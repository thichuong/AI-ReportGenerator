import os
import gc
import threading
import time
from datetime import datetime, timezone, timedelta
import pytz
from google import genai
from google.genai import types
from .report_workflow import generate_auto_research_report_langgraph
from .progress_tracker import progress_tracker





def generate_auto_research_report(api_key, max_attempts=3):
    """
    Wrapper function cho LangGraph workflow V2 để giữ tương thích với code cũ.
    
    Args:
        api_key (str): API key của Gemini
        max_attempts (int): Số lần thử tối đa để tạo báo cáo PASS
        
    Returns:
        dict: Kết quả workflow với success status và report_id
    """
    try:
        result = generate_auto_research_report_langgraph(api_key, max_attempts)
        
        # Đảm bảo trả về dict format
        if isinstance(result, dict):
            return result
        else:
            # Fallback cho backward compatibility
            return {'success': bool(result), 'report_id': None}
            
    except Exception as e:
        print(f"❌ Error in generate_auto_research_report: {e}")
        return {
            'success': False, 
            'error': str(e),
            'report_id': None
        }


def schedule_auto_report(api_key, schedule_times=None):
    """
    Lên lịch tự động tạo báo cáo theo khung giờ cố định hàng ngày.
    
    Args:
        api_key (str): API key của Gemini
        schedule_times (list): List các khung giờ chạy báo cáo (format: "HH:MM")
                              Mặc định: ["07:30", "15:00", "19:00"] (giờ Việt Nam)
    """
    if schedule_times is None:
        schedule_times = ["07:30", "15:00", "19:00"]  # Khung giờ mặc định
    
    # Timezone Việt Nam
    vietnam_tz = pytz.timezone('Asia/Ho_Chi_Minh')
    
    def get_next_run_time():
        """Tính toán thời gian chạy tiếp theo."""
        now = datetime.now(vietnam_tz)
        today = now.date()
        
        # Tìm khung giờ tiếp theo trong ngày
        for time_str in schedule_times:
            hour, minute = map(int, time_str.split(':'))
            scheduled_time = vietnam_tz.localize(
                datetime.combine(today, datetime.min.time().replace(hour=hour, minute=minute))
            )
            
            if scheduled_time > now:
                return scheduled_time
        
        # Nếu không có khung giờ nào còn lại trong ngày, lấy khung đầu tiên của ngày mai
        tomorrow = today + timedelta(days=1)
        hour, minute = map(int, schedule_times[0].split(':'))
        next_run = vietnam_tz.localize(
            datetime.combine(tomorrow, datetime.min.time().replace(hour=hour, minute=minute))
        )
        return next_run
    
    def run_scheduler():
        """Chạy scheduler theo lịch xếp sẵn - KHÔNG tự động retry sau thất bại"""
        print(f"[{datetime.now(vietnam_tz)}] 🎯 Auto report scheduler khởi động với khung giờ: {', '.join(schedule_times)} (GMT+7)")

        while True:
            try:
                # Tính toán thời gian chạy tiếp theo (CHỈ dựa trên lịch xếp sẵn)
                now = datetime.now(vietnam_tz)
                next_run = get_next_run_time()

                wait_seconds = (next_run - now).total_seconds()

                if wait_seconds > 0:
                    print(f"[{now}] ⏰ Scheduler: Chạy tiếp theo vào {next_run.strftime('%Y-%m-%d %H:%M:%S %Z')}")
                    time.sleep(wait_seconds)

                # Kiểm tra lại thời gian sau khi sleep (có thể bị drift)
                now = datetime.now(vietnam_tz)
                current_time = now.strftime("%H:%M")

                # Kiểm tra xem có đúng khung giờ không (với tolerance 5 phút)
                should_run = False
                for time_str in schedule_times:
                    scheduled_hour, scheduled_minute = map(int, time_str.split(':'))
                    scheduled_time = now.replace(hour=scheduled_hour, minute=scheduled_minute, second=0, microsecond=0)
                    time_diff = abs((now - scheduled_time).total_seconds())

                    if time_diff <= 300:  # Tolerance 5 phút
                        should_run = True
                        break

                if not should_run:
                    print(f"[{now}] ⚠️ Scheduler: Không đúng khung giờ, skip...")
                    continue

                print(f"[{now}] 🚀 Scheduler: Bắt đầu tạo báo cáo tự động (khung {current_time})...")
                start_time = datetime.now(vietnam_tz)

                # Chạy tạo báo cáo với số lần thử tối đa
                max_attempts = int(os.getenv('MAX_REPORT_ATTEMPTS', '3'))
                result = generate_auto_research_report(api_key, max_attempts)

                if isinstance(result, dict) and result.get('success'):
                    end_time = datetime.now(vietnam_tz)
                    duration = (end_time - start_time).total_seconds()
                    report_id = result.get('report_id')
                    print(f"[{end_time}] ✅ Scheduler: Báo cáo #{report_id} tạo thành công trong {duration:.1f}s (khung {current_time})")

                    # Giải phóng bộ nhớ sau khi hoàn thành báo cáo
                    del result
                    gc.collect()
                    print(f"[{datetime.now(vietnam_tz)}] 🧹 Memory cleanup completed")

                elif isinstance(result, bool) and result:
                    end_time = datetime.now(vietnam_tz)
                    duration = (end_time - start_time).total_seconds()
                    print(f"[{end_time}] ✅ Scheduler: Báo cáo tạo thành công trong {duration:.1f}s (khung {current_time})")

                    # Giải phóng bộ nhớ
                    del result
                    gc.collect()
                    print(f"[{datetime.now(vietnam_tz)}] 🧹 Memory cleanup completed")
                else:
                    error_info = ""
                    if isinstance(result, dict) and result.get('errors'):
                        error_info = f" - Errors: {result['errors'][:2]}"  # Show first 2 errors

                    end_time = datetime.now(vietnam_tz)
                    duration = (end_time - start_time).total_seconds()
                    print(f"[{end_time}] ❌ Scheduler: Tạo báo cáo thất bại trong {duration:.1f}s (khung {current_time}){error_info}")
                    print(f"[{end_time}] ℹ️ Scheduler: Sẽ thử lại vào khung giờ tiếp theo theo lịch xếp sẵn")

                    # Giải phóng bộ nhớ ngay cả khi thất bại
                    del result
                    gc.collect()

            except Exception as e:
                print(f"[{datetime.now(vietnam_tz)}] ❌ Scheduler error: {e}")
                print(f"[{datetime.now(vietnam_tz)}] ℹ️ Scheduler: Sẽ tiếp tục chạy vào khung giờ tiếp theo theo lịch xếp sẵn")

                # Giải phóng bộ nhớ khi có exception
                gc.collect()
    
    # Tạo và khởi động thread cho scheduler
    scheduler_thread = threading.Thread(target=run_scheduler, daemon=True)
    scheduler_thread.start()


def start_auto_report_scheduler():
    """
    Khởi động scheduler tự động tạo báo cáo theo khung giờ cố định.
    Hàm này sẽ được gọi khi FastAPI app khởi động.
    
    Returns:
        bool: True nếu scheduler được khởi động thành công, False nếu không
    """
    # Lấy API key từ environment variables
    api_key = os.getenv('GEMINI_API_KEY')
    
    if not api_key:
        print("WARNING: GEMINI_API_KEY không được thiết lập. Auto report scheduler không được khởi động.")
        return False
    
    # Kiểm tra nếu đang chạy trên môi trường production hoặc có enable scheduler
    enable_scheduler = os.getenv('ENABLE_AUTO_REPORT_SCHEDULER', 'false').lower() == 'true'
    
    if enable_scheduler:
        # Lấy custom schedule times từ environment variable (optional)
        custom_times = os.getenv('AUTO_REPORT_SCHEDULE_TIMES')
        schedule_times = None
        
        if custom_times:
            # Parse custom times: "07:30,15:00,19:00"
            schedule_times = [time.strip() for time in custom_times.split(',')]
            print(f"INFO: Sử dụng khung giờ custom: {schedule_times}")
        else:
            print("INFO: Sử dụng khung giờ mặc định: 07:30, 15:00, 19:00 (GMT+7)")
        
        schedule_auto_report(api_key, schedule_times)
        return True
    else:
        print("INFO: Auto report scheduler chưa được bật. Thiết lập ENABLE_AUTO_REPORT_SCHEDULER=true để bật.")
        return False


def create_manual_report():
    """
    Tạo báo cáo thủ công ngay lập tức.
    Hàm này có thể được gọi từ API endpoint hoặc command line.
    
    Returns:
        dict: Kết quả tạo báo cáo
    """
    api_key = os.getenv('GEMINI_API_KEY')
    
    if not api_key:
        return {
            'success': False,
            'error': 'GEMINI_API_KEY không được thiết lập'
        }
    
    # Timezone Việt Nam
    vietnam_tz = pytz.timezone('Asia/Ho_Chi_Minh')
    
    print(f"[{datetime.now(vietnam_tz)}] 🚀 Manual report generation started...")
    start_time = datetime.now(vietnam_tz)
    
    try:
        max_attempts = int(os.getenv('MAX_REPORT_ATTEMPTS', '3'))
        result = generate_auto_research_report(api_key, max_attempts)
        
        end_time = datetime.now(vietnam_tz)
        duration = (end_time - start_time).total_seconds()
        
        if isinstance(result, dict) and result.get('success'):
            report_id = result.get('report_id')
            print(f"[{end_time}] ✅ Manual report #{report_id} created successfully in {duration:.1f}s")
            
            response = {
                'success': True,
                'report_id': report_id,
                'duration': duration,
                'message': f'Báo cáo #{report_id} được tạo thành công trong {duration:.1f}s'
            }
            
            # Giải phóng bộ nhớ
            del result
            gc.collect()
            print(f"[{datetime.now(vietnam_tz)}] 🧹 Memory cleanup completed")
            
            return response
        else:
            error_msg = 'Tạo báo cáo thất bại'
            if isinstance(result, dict) and result.get('errors'):
                error_msg = f"Tạo báo cáo thất bại: {result['errors'][0] if result['errors'] else 'Unknown error'}"
            
            print(f"[{end_time}] ❌ Manual report failed in {duration:.1f}s: {error_msg}")
            
            response = {
                'success': False,
                'duration': duration,
                'error': error_msg
            }
            
            # Giải phóng bộ nhớ
            del result
            gc.collect()
            
            return response
            
    except Exception as e:
        end_time = datetime.now(vietnam_tz)
        duration = (end_time - start_time).total_seconds()
        error_msg = f"Lỗi khi tạo báo cáo: {str(e)}"
        print(f"[{end_time}] ❌ Manual report failed in {duration:.1f}s: {error_msg}")
        
        # Giải phóng bộ nhớ
        gc.collect()
        
        return {
            'success': False,
            'duration': duration,
            'error': error_msg
        }
