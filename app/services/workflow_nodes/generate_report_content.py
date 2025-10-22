"""
Node để soạn nội dung báo cáo markdown từ nội dung nghiên cứu
"""
import time
from google.genai import types
from .base import ReportState, read_prompt_file, get_prompt_from_env
from ...services.progress_tracker import progress_tracker


def generate_report_content_node(state: ReportState) -> ReportState:
    """Node để chuyển nội dung nghiên cứu thành báo cáo phân tích chuyên sâu (markdown)"""
    session_id = state["session_id"]
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
        max_output_tokens=10000,
    )

    # Retry cho bước soạn báo cáo
    for attempt in range(3):
        try:
            progress_tracker.update_step(session_id, details=f"Gọi AI soạn báo cáo (lần {attempt+1}/3)...")
            response = state["client"].models.generate_content(
                model=state["model"],
                contents=contents,
                config=config
            )
            break
        except Exception as err:
            if attempt < 2:
                wait_time = (attempt + 1) * 20
                progress_tracker.update_step(session_id, details=f"Lỗi soạn báo cáo, chờ {wait_time}s...")
                time.sleep(wait_time)
            else:
                error_msg = f"Không thể soạn báo cáo sau 3 lần thử: {err}"
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
