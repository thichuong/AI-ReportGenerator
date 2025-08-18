import time
import logging
from datetime import datetime
from typing import Dict, Any
from threading import Lock

# Thiết lập logger cho progress tracker
logger = logging.getLogger(__name__)
logger.setLevel(logging.INFO)

# Tạo formatter cho logs
formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')

# Tạo console handler nếu chưa có
if not logger.handlers:
    console_handler = logging.StreamHandler()
    console_handler.setLevel(logging.INFO)
    console_handler.setFormatter(formatter)
    logger.addHandler(console_handler)

class ProgressTracker:
    """Đơn giản hóa theo dõi tiến độ - chỉ hiển thị step chính với substep queue"""
    
    def __init__(self):
        self.current_progress = {}
        self.lock = Lock()
        
    def start_progress(self, session_id: str, total_steps: int = 9):
        """Bắt đầu theo dõi tiến độ cho một session. Có thể truyền vào số bước (total_steps) động."""
        logger.info(f"Starting progress tracking for session: {session_id} | Total steps: {total_steps}")
        
        with self.lock:
            self.current_progress[session_id] = {
                'step': 0,
                'total_steps': total_steps,
                'current_step_name': 'Khởi tạo...',
                'percentage': 0,
                'status': 'running',
                'start_time': time.time(),
                'details': '',
                'last_update': time.time()
            }
        
        logger.info(f"Progress tracking initialized successfully for session: {session_id}")
    
    def update_step(self, session_id: str, step: int = None, step_name: str = None, details: str = ''):
        """Cập nhật progress - gộp step và substep thành một"""
        timestamp = datetime.now().strftime("[%H:%M:%S.%f]")[:-3]  # Include milliseconds
        
        with self.lock:
            if session_id in self.current_progress:
                progress = self.current_progress[session_id]
                
                # Nếu có step number và step_name, đây là major step
                if step is not None and step_name is not None:
                    progress['step'] = step
                    progress['current_step_name'] = f"{timestamp} 🔄 Bước {step}: {step_name}"
                    progress['percentage'] = int((step / progress['total_steps']) * 100)
                    progress['details'] = f"{timestamp} {details}" if details else ""
                    progress['last_update'] = time.time()
                    
                    # Log thông tin step chính
                    logger.info(f"Session {session_id} - Step {step}/{progress['total_steps']} ({progress['percentage']}%): {step_name}")
                    if details:
                        logger.debug(f"Session {session_id} - Step details: {details}")
                
                # Nếu chỉ có details, đây là log entry detail
                elif details is not None:
                    timestamped_details = f"{timestamp} {details}"
                    progress['details'] = timestamped_details
                    progress['last_update'] = time.time()
                    
                    # Log chi tiết
                    logger.info(f"Session {session_id} - Detail: {details}")
            else:
                logger.warning(f"Session {session_id} not found in progress tracker")
    
    def update_substep(self, session_id: str, details: str):
        """Backward compatibility - gọi update_step với chỉ details"""
        self.update_step(session_id, details=details)
    
    def get_progress(self, session_id: str) -> Dict[str, Any]:
        """Lấy tiến độ hiện tại của một session"""
        with self.lock:
            return self.current_progress.get(session_id, None)
    
    def complete_progress(self, session_id: str, success: bool = True, report_id: int = None):
        """Hoàn thành tiến độ"""
        timestamp = datetime.now().strftime("[%H:%M:%S.%f]")[:-3]  # Include milliseconds
        
        with self.lock:
            if session_id in self.current_progress:
                progress = self.current_progress[session_id]
                progress['step'] = progress['total_steps']
                progress['percentage'] = 100
                progress['status'] = 'completed' if success else 'error'
                progress['current_step_name'] = f"{timestamp} ✅ Hoàn thành!" if success else f"{timestamp} ❌ Có lỗi xảy ra"
                progress['report_id'] = report_id
                progress['end_time'] = time.time()
                progress['last_update'] = time.time()
                
                # Tính thời gian thực hiện
                duration = progress['end_time'] - progress['start_time']
                minutes = int(duration // 60)
                seconds = int(duration % 60)
                
                if success:
                    logger.info(f"Session {session_id} completed successfully in {minutes}m {seconds}s"
                              + (f" | Report ID: {report_id}" if report_id else ""))
                else:
                    logger.error(f"Session {session_id} completed with errors after {minutes}m {seconds}s")
            else:
                logger.warning(f"Session {session_id} not found when trying to complete progress")
    
    def error_progress(self, session_id: str, error_msg: str):
        """Báo lỗi trong quá trình"""
        timestamp = datetime.now().strftime("[%H:%M:%S.%f]")[:-3]  # Include milliseconds
        
        with self.lock:
            if session_id in self.current_progress:
                progress = self.current_progress[session_id]
                progress['status'] = 'error'
                progress['current_step_name'] = f"{timestamp} ❌ Lỗi"
                progress['details'] = f"{timestamp} {error_msg}"
                progress['end_time'] = time.time()
                progress['last_update'] = time.time()
                
                # Tính thời gian thực hiện trước khi lỗi
                duration = progress['end_time'] - progress['start_time']
                minutes = int(duration // 60)
                seconds = int(duration % 60)
                
                logger.error(f"Session {session_id} encountered error after {minutes}m {seconds}s: {error_msg}")
            else:
                logger.warning(f"Session {session_id} not found when trying to log error: {error_msg}")

# Global instance
progress_tracker = ProgressTracker()
