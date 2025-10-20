import os
import gc
import threading
import time
from datetime import datetime, timezone, timedelta
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


def schedule_auto_report(api_key, interval_hours=6):
    """
    Lên lịch tự động tạo báo cáo mỗi interval_hours giờ với improved error handling.
    
    Args:
        api_key (str): API key của Gemini
        interval_hours (int): Khoảng thời gian giữa các lần tạo báo cáo (giờ)
    """
    def run_scheduler():
        consecutive_failures = 0
        max_consecutive_failures = 3
        
        while True:
            try:
                start_time = datetime.now()
                print(f"[{start_time}] 🚀 Scheduler: Bắt đầu tạo báo cáo tự động...")
                
                # Chạy tạo báo cáo với số lần thử tối đa
                max_attempts = int(os.getenv('MAX_REPORT_ATTEMPTS', '3'))
                
                result = generate_auto_research_report(api_key, max_attempts)
                
                if isinstance(result, dict) and result.get('success'):
                    consecutive_failures = 0  # Reset failure counter
                    end_time = datetime.now()
                    duration = (end_time - start_time).total_seconds()
                    report_id = result.get('report_id')
                    print(f"[{end_time}] ✅ Scheduler: Báo cáo #{report_id} tạo thành công trong {duration:.1f}s")
                    
                    # Giải phóng bộ nhớ sau khi hoàn thành báo cáo
                    del result
                    gc.collect()
                    print(f"[{datetime.now()}] 🧹 Memory cleanup completed")
                    
                elif isinstance(result, bool) and result:
                    consecutive_failures = 0  # Reset failure counter  
                    end_time = datetime.now()
                    duration = (end_time - start_time).total_seconds()
                    print(f"[{end_time}] ✅ Scheduler: Báo cáo tạo thành công trong {duration:.1f}s")
                    
                    # Giải phóng bộ nhớ
                    del result
                    gc.collect()
                    print(f"[{datetime.now()}] 🧹 Memory cleanup completed")
                else:
                    consecutive_failures += 1
                    error_info = ""
                    if isinstance(result, dict) and result.get('errors'):
                        error_info = f" - Errors: {result['errors'][:2]}"  # Show first 2 errors
                    
                    print(f"[{datetime.now()}] ❌ Scheduler: Tạo báo cáo thất bại ({consecutive_failures}/{max_consecutive_failures}){error_info}")
                    
                    # Giải phóng bộ nhớ ngay cả khi thất bại
                    del result
                    gc.collect()
                    
                    # Nếu thất bại liên tiếp quá nhiều, tăng interval
                    if consecutive_failures >= max_consecutive_failures:
                        extended_interval = interval_hours * 2
                        print(f"[{datetime.now()}] ⚠️ Scheduler: Too many failures, extending interval to {extended_interval}h")
                        time.sleep(extended_interval * 3600)
                        consecutive_failures = 0  # Reset counter
                        continue
                    
            except Exception as e:
                consecutive_failures += 1
                print(f"[{datetime.now()}] ❌ Scheduler error ({consecutive_failures}/{max_consecutive_failures}): {e}")
                
                # Giải phóng bộ nhớ khi có exception
                gc.collect()
                
                # Nếu lỗi liên tiếp quá nhiều, restart scheduler
                if consecutive_failures >= max_consecutive_failures:
                    print(f"[{datetime.now()}] 🔄 Scheduler: Restarting due to consecutive failures...")
                    time.sleep(300)  # Wait 5 minutes before restart
                    consecutive_failures = 0
                    continue
            
            # Chờ interval_hours giờ trước khi chạy lần tiếp theo
            next_run = datetime.now().replace(microsecond=0) + timedelta(hours=interval_hours)
            print(f"[{datetime.now()}] ⏰ Scheduler: Next run scheduled at {next_run}")
            time.sleep(interval_hours * 3600)
    
    # Tạo và khởi động thread cho scheduler
    scheduler_thread = threading.Thread(target=run_scheduler, daemon=True)
    scheduler_thread.start()
    print(f"[{datetime.now()}] 🎯 Auto report scheduler started (interval: {interval_hours}h, max failures: 3)")


def start_auto_report_scheduler():
    """
    Khởi động scheduler tự động tạo báo cáo.
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
        # Lấy interval từ environment variable, mặc định là 3 giờ
        interval_hours = int(os.getenv('AUTO_REPORT_INTERVAL_HOURS', '3'))
        schedule_auto_report(api_key, interval_hours)
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
    
    print(f"[{datetime.now()}] 🚀 Manual report generation started...")
    start_time = datetime.now()
    
    try:
        max_attempts = int(os.getenv('MAX_REPORT_ATTEMPTS', '3'))
        result = generate_auto_research_report(api_key, max_attempts)
        
        end_time = datetime.now()
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
            print(f"[{datetime.now()}] 🧹 Memory cleanup completed")
            
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
        end_time = datetime.now()
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
