import time
import gc
import psycopg2
from sqlalchemy.exc import OperationalError
from .base import ReportState
from ...services.progress_tracker import progress_tracker
from ...db.session import SessionLocal
from ...models import CryptoReport as Report


def save_report_to_database(state: ReportState) -> ReportState:
    """Save report to database with retry logic for SSL errors"""
    session_id = state.get("session_id")
    max_retries = 3
    session = None
    
    # Validate required data
    if "error_messages" not in state:
        state["error_messages"] = []
    
    # Kiểm tra tất cả dữ liệu có đủ không
    required_fields = {
        "html_content": "HTML content",
        "css_content": "CSS content", 
        "js_content": "JavaScript content",
        "html_content_en": "English HTML content",
        "js_content_en": "English JavaScript content"
    }
    
    missing_fields = []
    empty_fields = []
    
    for field, description in required_fields.items():
        if field not in state:
            missing_fields.append(description)
        elif not state.get(field) or (isinstance(state.get(field), str) and len(state.get(field).strip()) == 0):
            empty_fields.append(description)
    
    if missing_fields or empty_fields:
        error_messages = []
        if missing_fields:
            error_messages.append(f"Missing fields: {', '.join(missing_fields)}")
        if empty_fields:
            error_messages.append(f"Empty fields: {', '.join(empty_fields)}")
        
        error_msg = "Cannot save incomplete report. " + "; ".join(error_messages)
        state["error_messages"].append(error_msg)
        state["success"] = False
        progress_tracker.error_progress(session_id, error_msg)
        print(f"Validation Error: {error_msg}")
        return state
    
    for attempt in range(max_retries):
        try:
            if attempt > 0:
                progress_tracker.update_step(session_id, details=f"Retrying database save (attempt {attempt + 1}/{max_retries})...")
                time.sleep(2 ** attempt)
            else:
                progress_tracker.update_step(session_id, details="Creating new report record...")
            
            session = SessionLocal()
            
            new_report = Report(
                html_content=state["html_content"],
                css_content=state.get("css_content", ""),
                js_content=state.get("js_content", ""),
                html_content_en=state.get("html_content_en"),
                js_content_en=state.get("js_content_en"),
            )
            
            progress_tracker.update_step(session_id, details="Committing to database...")
            
            session.add(new_report)
            session.commit()
            
            state["report_id"] = new_report.id
            state["success"] = True
            
            progress_tracker.complete_progress(session_id, True, new_report.id)
            print(f"Database save successful - Report ID: {new_report.id}")
            
            session.close()
            
            # Giải phóng bộ nhớ sau khi lưu thành công
            # Xóa các nội dung lớn khỏi state để giảm memory footprint
            large_fields = ["html_content", "css_content", "js_content", 
                          "html_content_en", "js_content_en"]
            for field in large_fields:
                if field in state:
                    del state[field]
            
            # Thu hồi bộ nhớ
            del new_report
            gc.collect()
            print(f"✅ Memory cleaned up after database save")
            
            return state
            
        except (OperationalError, psycopg2.OperationalError) as e:
            error_detail = str(e)
            is_ssl_error = any(keyword in error_detail.lower() for keyword in [
                'ssl', 'decryption failed', 'bad record mac', 'connection reset'
            ])
            
            if session:
                try:
                    session.rollback()
                    session.close()
                except:
                    pass
                session = None
            
            if is_ssl_error and attempt < max_retries - 1:
                wait_time = 2 ** attempt
                print(f"SSL error detected, retrying in {wait_time}s...")
                progress_tracker.update_step(session_id, details=f"SSL error - retry in {wait_time}s...")
                # Giải phóng bộ nhớ trước khi retry
                gc.collect()
                time.sleep(wait_time)
                continue
            else:
                error_msg = f"Database save error after {max_retries} attempts: {error_detail}"
                state["error_messages"].append(error_msg)
                state["success"] = False
                progress_tracker.error_progress(session_id, error_msg)
                print(f"Error: {error_msg}")
                return state
                
        except Exception as e:
            if session:
                try:
                    session.rollback()
                    session.close()
                except:
                    pass
                session = None
            
            error_msg = f"Non-retryable database error: {e}"
            state["error_messages"].append(error_msg)
            state["success"] = False
            progress_tracker.error_progress(session_id, error_msg)
            print(f"Error: {error_msg}")
            
            # Giải phóng bộ nhớ khi có lỗi
            gc.collect()
            return state
    
    # If we get here, all attempts failed
    if session:
        try:
            session.rollback()
            session.close()
        except:
            pass
    
    error_msg = f"Failed to save to database after {max_retries} attempts"
    state["error_messages"].append(error_msg)
    state["success"] = False
    progress_tracker.error_progress(session_id, error_msg)
    print(f"Error: {error_msg}")
    
    # Giải phóng bộ nhớ khi thất bại
    gc.collect()
    return state


def save_database_node(state: ReportState) -> ReportState:
    """Node function for LangGraph workflow"""
    return save_report_to_database(state)
