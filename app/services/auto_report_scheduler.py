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
        consecutive_failures = 0
        max_consecutive_failures = 3
        retry_run_time = None  # Biến lưu thời gian retry sau 30p
        
        print(f"[{datetime.now(vietnam_tz)}] 🎯 Auto report scheduler khởi động với khung giờ: {', '.join(schedule_times)} (GMT+7)")
        
        while True:
            try:
                # Tính toán thời gian chạy tiếp theo
                now = datetime.now(vietnam_tz)
                scheduled_next_run = get_next_run_time()
                
                # So sánh retry_run_time (30p) và scheduled_next_run, chọn cái nào sớm hơn
                if retry_run_time is not None:
                    next_run = min(retry_run_time, scheduled_next_run)
                    if next_run == retry_run_time:
                        print(f"[{now}] 🔄 Sử dụng retry time (30p) thay vì scheduled time")
                else:
                    next_run = scheduled_next_run
                
                wait_seconds = (next_run - now).total_seconds()
                
                if wait_seconds > 0:
                    print(f"[{now}] ⏰ Scheduler: Chạy tiếp theo vào {next_run.strftime('%Y-%m-%d %H:%M:%S %Z')}")
                    time.sleep(wait_seconds)
                
                # Kiểm tra lại thời gian sau khi sleep (có thể bị drift)
                now = datetime.now(vietnam_tz)
                current_time = now.strftime("%H:%M")
                
                # Nếu đang retry (có retry_run_time), không cần check khung giờ
                is_retry = (retry_run_time is not None and abs((now - retry_run_time).total_seconds()) <= 300)
                
                if not is_retry:
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
                        retry_run_time = None  # Reset retry time nếu skip
                        continue
                else:
                    print(f"[{now}] 🔄 Scheduler: Retry run - skipping schedule check")
                
                # Reset retry_run_time trước khi chạy báo cáo
                retry_run_time = None
                
                print(f"[{now}] 🚀 Scheduler: Bắt đầu tạo báo cáo tự động (khung {current_time})...")
                start_time = datetime.now(vietnam_tz)
                
                # Chạy tạo báo cáo với số lần thử tối đa
                max_attempts = int(os.getenv('MAX_REPORT_ATTEMPTS', '3'))
                result = generate_auto_research_report(api_key, max_attempts)
                
                if isinstance(result, dict) and result.get('success'):
                    consecutive_failures = 0  # Reset failure counter
                    retry_run_time = None  # Reset retry time khi thành công
                    end_time = datetime.now(vietnam_tz)
                    duration = (end_time - start_time).total_seconds()
                    report_id = result.get('report_id')
                    print(f"[{end_time}] ✅ Scheduler: Báo cáo #{report_id} tạo thành công trong {duration:.1f}s (khung {current_time})")
                    
                    # Giải phóng bộ nhớ sau khi hoàn thành báo cáo
                    del result
                    gc.collect()
                    print(f"[{datetime.now(vietnam_tz)}] 🧹 Memory cleanup completed")
                    
                elif isinstance(result, bool) and result:
                    consecutive_failures = 0  # Reset failure counter
                    retry_run_time = None  # Reset retry time khi thành công
                    end_time = datetime.now(vietnam_tz)
                    duration = (end_time - start_time).total_seconds()
                    print(f"[{end_time}] ✅ Scheduler: Báo cáo tạo thành công trong {duration:.1f}s (khung {current_time})")
                    
                    # Giải phóng bộ nhớ
                    del result
                    gc.collect()
                    print(f"[{datetime.now(vietnam_tz)}] 🧹 Memory cleanup completed")
                else:
                    consecutive_failures += 1
                    error_info = ""
                    if isinstance(result, dict) and result.get('errors'):
                        error_info = f" - Errors: {result['errors'][:2]}"  # Show first 2 errors
                    
                    print(f"[{datetime.now(vietnam_tz)}] ❌ Scheduler: Tạo báo cáo thất bại ({consecutive_failures}/{max_consecutive_failures}){error_info}")
                    
                    # Giải phóng bộ nhớ ngay cả khi thất bại
                    del result
                    gc.collect()
                    
                    # Nếu thất bại liên tiếp quá nhiều, skip tới khung giờ tiếp theo
                    if consecutive_failures >= max_consecutive_failures:
                        print(f"[{datetime.now(vietnam_tz)}] ⚠️ Scheduler: Too many failures, skipping to next scheduled time...")
                        consecutive_failures = 0  # Reset counter
                        retry_run_time = None  # Reset retry time
                        continue
                    
                    # Thiết lập retry time là 30 phút từ bây giờ
                    retry_wait_minutes = 30
                    retry_run_time = datetime.now(vietnam_tz) + timedelta(minutes=retry_wait_minutes)
                    print(f"[{datetime.now(vietnam_tz)}] 🔄 Scheduler: Đặt retry time sau {retry_wait_minutes} phút vào {retry_run_time.strftime('%H:%M:%S')}")
                    continue  # Quay lại vòng lặp để tính toán next_run mới
                    
            except Exception as e:
                consecutive_failures += 1
                print(f"[{datetime.now(vietnam_tz)}] ❌ Scheduler error ({consecutive_failures}/{max_consecutive_failures}): {e}")
                
                # Giải phóng bộ nhớ khi có exception
                gc.collect()
                
                # Nếu lỗi liên tiếp quá nhiều, chờ tới khung giờ tiếp theo
                if consecutive_failures >= max_consecutive_failures:
                    print(f"[{datetime.now(vietnam_tz)}] 🔄 Scheduler: Too many errors, waiting for next scheduled time...")
                    consecutive_failures = 0
                    retry_run_time = None  # Reset retry time
                    continue
                
                # Thiết lập retry time là 30 phút từ bây giờ
                retry_wait_minutes = 30
                retry_run_time = datetime.now(vietnam_tz) + timedelta(minutes=retry_wait_minutes)
                print(f"[{datetime.now(vietnam_tz)}] 🔄 Scheduler: Đặt retry time sau {retry_wait_minutes} phút vào {retry_run_time.strftime('%H:%M:%S')}")
                continue  # Quay lại vòng lặp để tính toán next_run mới
    
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
