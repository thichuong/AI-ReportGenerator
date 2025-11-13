"""
Node tạo giao diện từ báo cáo nghiên cứu
"""
import re
from google.genai import types
from .base import ReportState, read_prompt_file, get_prompt_from_env, call_gemini_with_rate_limit_handling
from ...services.progress_tracker import progress_tracker


def create_interface_node(state: ReportState) -> ReportState:
    """Node để tạo giao diện từ báo cáo nghiên cứu"""
    session_id = state["session_id"]
    interface_attempt_key = "interface_attempt"
    if interface_attempt_key not in state:
        state[interface_attempt_key] = 0
    state[interface_attempt_key] += 1
    
    progress_tracker.update_step(session_id, 5, f"Tạo giao diện (lần {state[interface_attempt_key]})", "Chuẩn bị tạo HTML, CSS, JS")
    report_md = state.get('report_content') or state.get('research_content', '')
    create_report_prompt = get_prompt_from_env('create_report')
    # Tạo request đầy đủ
    full_request = f"{create_report_prompt}\n\n---\n\n**NỘI DUNG BÁO CÁO CẦN XỬ LÝ:**\n\n{report_md}"
    
    interface_contents = [
        types.Content(
            role="user",
            parts=[
                types.Part.from_text(text=full_request),
            ],
        ),
    ]
    
    simple_config = types.GenerateContentConfig(
        temperature=0,
        candidate_count=1,
    )
    
    # Call API with centralized error handler
    progress_tracker.update_step(session_id, details="Gọi AI tạo giao diện...")
    interface_response, error_msg, is_rate_limit = call_gemini_with_rate_limit_handling(
        client=state["client"],
        model=state["model"],
        contents=interface_contents,
        config=simple_config,
        session_id=session_id,
        operation_name="create_interface",
        max_retries=3
    )

    # Check for rate limit error - stop immediately
    if is_rate_limit:
        state["error_messages"].append(error_msg)
        state["success"] = False
        progress_tracker.error_progress(session_id, "🚫 Rate limit error - dừng workflow ngay lập tức")
        return state

    # Check for other errors after retries
    if error_msg:
        state["error_messages"].append(error_msg)
        state["success"] = False
        progress_tracker.error_progress(session_id, error_msg)
        return state
    
    # Kiểm tra interface response
    if not interface_response or not hasattr(interface_response, 'text'):
        error_msg = "Interface response không hợp lệ từ AI"
        state["error_messages"].append(error_msg)
        state["success"] = False
        progress_tracker.error_progress(session_id, error_msg)
        return state
        
    if not interface_response.text:
        error_msg = "Không nhận được nội dung interface từ AI"
        state["error_messages"].append(error_msg)
        state["success"] = False
        progress_tracker.error_progress(session_id, error_msg)
        return state
    
    state["interface_content"] = interface_response.text
    state["success"] = True
    progress_tracker.update_step(session_id, details="✓ Tạo giao diện hoàn thành")
    
    # 🧹 Memory cleanup - giải phóng temporary large objects
    del full_request  # Xóa prompt + report content (có thể 100KB+)
    del interface_contents  # Xóa request contents
    del interface_response  # Xóa response object với HTML/CSS/JS
    import gc
    gc.collect()
    print("🧹 [create_interface] Memory cleanup completed")
    
    return state
