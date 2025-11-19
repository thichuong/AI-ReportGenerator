"""
Node để soạn nội dung báo cáo markdown từ nội dung nghiên cứu
"""
from google.genai import types
from .base import ReportState, read_prompt_file, get_prompt_from_env, call_gemini_with_rate_limit_handling
from ...services.progress_tracker import progress_tracker


def generate_report_content_node(state: ReportState) -> ReportState:
    """Node để chuyển nội dung nghiên cứu thành báo cáo phân tích chuyên sâu (markdown)"""
    session_id = state["session_id"]

    # CHECK RATE LIMIT FLAG - Skip node if already hit rate limit
    if state.get("rate_limit_stop"):
        print(f"⛔ [{session_id}] Skipping generate_report_content - rate limit flag is set")
        return state

    attempt_key = "report_attempt"
    if attempt_key not in state:
        state[attempt_key] = 0
    state[attempt_key] += 1

    # Bước soạn nội dung báo cáo
    progress_tracker.update_step(
        session_id,
        4,
        f"Soạn nội dung báo cáo (lần {state[attempt_key]})",
        "Tạo nội dung báo cáo markdown"
    )

    # Đọc prompt soạn báo cáo từ biến môi trường
    prompt = get_prompt_from_env('generate_report')
    if not prompt:
        error_msg = "Không thể đọc prompt soạn báo cáo từ biến môi trường"
        state["error_messages"].append(error_msg)
        state["success"] = False
        progress_tracker.error_progress(session_id, error_msg)
        return state

    # Tạo request để soạn báo cáo
    research_content = state.get("research_content", "")
    full_request = prompt.replace("{content}", research_content)

    contents = [
        types.Content(
            role="user",
            parts=[
                types.Part.from_text(text=full_request),
            ],
        ),
    ]

    config = types.GenerateContentConfig(
        temperature=0.5,
        candidate_count=1,
        max_output_tokens=25000,
        thinking_config=types.ThinkingConfig(
            thinking_budget=4096,
        ),
    )

    # Call API with centralized error handler
    progress_tracker.update_step(session_id, details="Gọi AI soạn báo cáo...")
    response, error_msg, is_rate_limit = call_gemini_with_rate_limit_handling(
        client=state["client"],
        model=state["model"],
        contents=contents,
        config=config,
        session_id=session_id,
        operation_name="generate_report_content",
        max_retries=3
    )

    # Check for rate limit error - stop immediately
    if is_rate_limit:
        state["error_messages"].append(error_msg)
        state["success"] = False
        state["rate_limit_stop"] = True  # SET FLAG to stop workflow
        progress_tracker.error_progress(session_id, "🚫 Rate limit error - đã set flag dừng workflow")
        print(f"⛔ [{session_id}] rate_limit_stop flag SET - workflow will terminate")
        return state

    # Check for other errors after retries
    if error_msg:
        state["error_messages"].append(error_msg)
        state["success"] = False
        progress_tracker.error_progress(session_id, error_msg)
        return state

    # Kiểm tra response
    if not response or not hasattr(response, 'text') or not response.text:
        error_msg = "Không nhận được nội dung báo cáo từ AI"
        state["error_messages"].append(error_msg)
        state["success"] = False
        progress_tracker.error_progress(session_id, error_msg)
        return state

    report_md = response.text.strip()
    state["report_content"] = report_md
    state["success"] = True
    progress_tracker.update_step(session_id, details=f"✓ Soạn báo cáo hoàn thành - {len(report_md)} chars")

    # 🧹 Memory cleanup - giải phóng temporary large objects
    del full_request  # Xóa prompt + research_content (có thể 50KB+)
    del contents  # Xóa request contents
    del response  # Xóa response object
    del report_md  # Xóa temporary variable (đã lưu vào state)
    import gc
    gc.collect()
    print("🧹 [generate_report] Memory cleanup completed")

    return state
