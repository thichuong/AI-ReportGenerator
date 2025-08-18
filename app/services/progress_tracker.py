import time
from datetime import datetime
from typing import Dict, Any
from threading import Lock

class ProgressTracker:
    """Đơn giản hóa theo dõi tiến độ - chỉ hiển thị step chính với substep queue"""
    
    def __init__(self):
        self.current_progress = {}
        self.lock = Lock()
        
    def start_progress(self, session_id: str, total_steps: int = 9):
        """Bắt đầu theo dõi tiến độ cho một session. Có thể truyền vào số bước (total_steps) động."""
        print(f"[PROGRESS] Starting session: {session_id} | total_steps={total_steps}")
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
                    print(f"[PROGRESS] Step {step}: {step_name}")
                
                # Nếu chỉ có details, đây là log entry detail
                elif details is not None:
                    timestamped_details = f"{timestamp} {details}"
                    progress['details'] = timestamped_details
                    progress['last_update'] = time.time()
                    print(f"[PROGRESS] Detail: {timestamped_details}")
    
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

# Global instance
progress_tracker = ProgressTracker()
