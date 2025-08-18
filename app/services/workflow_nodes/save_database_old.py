import time
import psycopg2
from sqlalchemy.exc import OperationalError
from .base import ReportState
from ...services.progress_tracker import progress_tracker
from ...db.session import SessionLocal
from ...models import CryptoReport as Report


def save_report_to_database(state: ReportState) -> ReportState:
    session_id = state.get("session_id")
    max_retries = 3
    session = None
    
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
                js_content_en=None
            )
            
            progress_tracker.update_step(session_id, details="Committing to database...")
            
            session.add(new_report)
            session.commit()
            
            state["report_id"] = new_report.id
            state["success"] = True
            
            progress_tracker.complete_progress(session_id, True, new_report.id)
            print(f"Database save successful - Report ID: {new_report.id}")
            
            session.close()
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
                time.sleep(wait_time)
                continue
            else:
                error_msg = f"Database save error after {max_retries} attempts: {error_detail}"
                if "error_messages" not in state:
                    state["error_messages"] = []
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
            if "error_messages" not in state:
                state["error_messages"] = []
            state["error_messages"].append(error_msg)
            state["success"] = False
            progress_tracker.error_progress(session_id, error_msg)
            print(f"Error: {error_msg}")
            return state
    
    if session:
        try:
            session.rollback()
            session.close()
        except:
            pass
    
    error_msg = f"Failed to save to database after {max_retries} attempts"
    if "error_messages" not in state:
        state["error_messages"] = []
    state["error_messages"].append(error_msg)
    state["success"] = False
    progress_tracker.error_progress(session_id, error_msg)
    print(f"Error: {error_msg}")
    return state


def save_database_node(state: ReportState) -> ReportState:
    return save_report_to_database(state)
