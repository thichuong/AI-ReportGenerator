"""
Node tạo giao diện theo từng thành phần riêng biệt (HTML, JS, CSS)
"""
import re
from google.genai import types
from .base import ReportState, read_prompt_file, get_prompt_from_env, call_gemini_with_rate_limit_handling
from ...services.progress_tracker import progress_tracker


def create_html_node(state: ReportState) -> ReportState:
    """Node để tạo HTML từ báo cáo nghiên cứu"""
    session_id = state["session_id"]
    html_attempt_key = "html_attempt"
    if html_attempt_key not in state:
        state[html_attempt_key] = 0
    state[html_attempt_key] += 1
    
    # Bước tạo HTML sau khi đã có nội dung báo cáo markdown
    progress_tracker.update_step(session_id, 5, f"Tạo HTML (lần {state[html_attempt_key]})", "Tạo cấu trúc HTML từ nội dung báo cáo")
    
    # Đọc prompt tạo HTML từ biến môi trường
    html_prompt = get_prompt_from_env('create_html')
    if not html_prompt:
        error_msg = "Không thể đọc prompt tạo HTML từ biến môi trường"
        state["error_messages"].append(error_msg)
        state["success"] = False
        progress_tracker.error_progress(session_id, error_msg)
        return state
    
    # Tạo request tạo HTML
    # Chuyển nội dung báo cáo markdown thành HTML semantic
    report_md = state.get('report_content') or state.get('research_content', '')
    full_request = f"{html_prompt}\n\n---\n\n**NỘI DUNG BÁO CÁO:**\n\n{report_md}"
    
    html_contents = [
        types.Content(
            role="user",
            parts=[
                types.Part.from_text(text=full_request),
            ],
        ),
    ]
    
    simple_config = types.GenerateContentConfig(
        temperature=0.1,
        candidate_count=1,
    )
    
    # Call API with centralized error handler
    progress_tracker.update_step(session_id, details="Gọi AI tạo HTML...")
    html_response, error_msg, is_rate_limit = call_gemini_with_rate_limit_handling(
        client=state["client"],
        model=state["model"],
        contents=html_contents,
        config=simple_config,
        session_id=session_id,
        operation_name="create_html",
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
    
    # Kiểm tra HTML response
    if not html_response or not hasattr(html_response, 'text') or not html_response.text:
        error_msg = "Không nhận được nội dung HTML từ AI"
        state["error_messages"].append(error_msg)
        state["success"] = False
        progress_tracker.error_progress(session_id, error_msg)
        return state
    
    # Trích xuất HTML content
    html_content = _extract_html(html_response.text)
    if not html_content:
        error_msg = "Không thể trích xuất HTML từ phản hồi AI"
        state["error_messages"].append(error_msg)
        state["success"] = False
        progress_tracker.error_progress(session_id, error_msg)
        return state
    
    state["html_content"] = html_content
    state["success"] = True
    progress_tracker.update_step(session_id, details=f"✓ Tạo HTML hoàn thành - {len(html_content)} chars")
    
    return state


def create_javascript_node(state: ReportState) -> ReportState:
    """Node để tạo JavaScript từ báo cáo nghiên cứu"""
    session_id = state["session_id"]
    js_attempt_key = "js_attempt"
    if js_attempt_key not in state:
        state[js_attempt_key] = 0
    state[js_attempt_key] += 1
    
    # Bước tạo JavaScript
    progress_tracker.update_step(session_id, 6, f"Tạo JavaScript (lần {state[js_attempt_key]})", "Tạo tương tác JS từ nội dung HTML")
    
    # Đọc prompt tạo JavaScript từ biến môi trường
    js_prompt = get_prompt_from_env('create_javascript')
    if not js_prompt:
        error_msg = "Không thể đọc prompt tạo JavaScript từ biến môi trường"
        state["error_messages"].append(error_msg)
        state["success"] = False
        progress_tracker.error_progress(session_id, error_msg)
        return state
    
    # Tạo request tạo JS (bao gồm HTML đã tạo để tương thích)
    html_context = state.get("html_content", "")
    full_request = f"{js_prompt}\n\n---\n\n**HTML ĐÃ TẠO:**\n\n{html_context}"
    
    js_contents = [
        types.Content(
            role="user",
            parts=[
                types.Part.from_text(text=full_request),
            ],
        ),
    ]
    
    simple_config = types.GenerateContentConfig(
        temperature=0.1,
        candidate_count=1,
    )
    
    # Call API with centralized error handler
    progress_tracker.update_step(session_id, details="Gọi AI tạo JavaScript...")
    js_response, error_msg, is_rate_limit = call_gemini_with_rate_limit_handling(
        client=state["client"],
        model=state["model"],
        contents=js_contents,
        config=simple_config,
        session_id=session_id,
        operation_name="create_javascript",
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
    
    # Kiểm tra JS response
    if not js_response or not hasattr(js_response, 'text') or not js_response.text:
        error_msg = "Không nhận được nội dung JavaScript từ AI"
        state["error_messages"].append(error_msg)
        state["success"] = False
        progress_tracker.error_progress(session_id, error_msg)
        return state
    
    # Trích xuất JS content
    js_content = _extract_javascript(js_response.text)
    if not js_content:
        js_content = "// JavaScript được tạo tự động\nconsole.log('Report loaded successfully');"
    
    state["js_content"] = js_content
    state["success"] = True
    progress_tracker.update_step(session_id, details=f"✓ Tạo JavaScript hoàn thành - {len(js_content)} chars")
    
    return state


def create_css_node(state: ReportState) -> ReportState:
    """Node để tạo CSS từ báo cáo nghiên cứu"""
    session_id = state["session_id"]
    css_attempt_key = "css_attempt"
    if css_attempt_key not in state:
        state[css_attempt_key] = 0
    state[css_attempt_key] += 1
    
    # Bước tạo CSS
    progress_tracker.update_step(session_id, 7, f"Tạo CSS (lần {state[css_attempt_key]})", "Tạo styling CSS từ nội dung HTML")
    
    # Đọc prompt tạo CSS từ biến môi trường
    css_prompt = get_prompt_from_env('create_css')
    if not css_prompt:
        error_msg = "Không thể đọc prompt tạo CSS từ biến môi trường"
        state["error_messages"].append(error_msg)
        state["success"] = False
        progress_tracker.error_progress(session_id, error_msg)
        return state
    
    # Tạo request tạo CSS (bao gồm HTML đã tạo để tương thích)
    html_context = state.get("html_content", "")
    full_request = f"{css_prompt}\n\n---\n\n**HTML ĐÃ TẠO:**\n\n{html_context}"
    
    css_contents = [
        types.Content(
            role="user",
            parts=[
                types.Part.from_text(text=full_request),
            ],
        ),
    ]
    
    simple_config = types.GenerateContentConfig(
        temperature=0.1,
        candidate_count=1,
    )
    
    # Call API with centralized error handler
    progress_tracker.update_step(session_id, details="Gọi AI tạo CSS...")
    css_response, error_msg, is_rate_limit = call_gemini_with_rate_limit_handling(
        client=state["client"],
        model=state["model"],
        contents=css_contents,
        config=simple_config,
        session_id=session_id,
        operation_name="create_css",
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
    
    # Kiểm tra CSS response
    if not css_response or not hasattr(css_response, 'text') or not css_response.text:
        error_msg = "Không nhận được nội dung CSS từ AI"
        state["error_messages"].append(error_msg)
        state["success"] = False
        progress_tracker.error_progress(session_id, error_msg)
        return state
    
    # Trích xuất CSS content
    css_content = _extract_css(css_response.text)
    if not css_content:
        css_content = "/* CSS được tạo tự động */\nbody { font-family: Arial, sans-serif; margin: 20px; }"
    
    state["css_content"] = css_content
    state["success"] = True
    progress_tracker.update_step(session_id, details=f"✓ Tạo CSS hoàn thành - {len(css_content)} chars")
    
    return state


def _extract_html(response_text):
    """Trích xuất nội dung HTML từ phản hồi"""
    if not response_text:
        return ""
    
    # Tìm khối HTML
    html_match = re.search(r"```html(.*?)```", response_text, re.DOTALL)
    if html_match:
        return html_match.group(1).strip()
    
    # Nếu không có khối mã, kiểm tra xem có HTML tags không
    if re.search(r'<html|<!doctype|<div|<body|<head', response_text, re.IGNORECASE):
        return response_text.strip()
    
    return ""


def _extract_javascript(response_text):
    """Trích xuất nội dung JavaScript từ phản hồi"""
    if not response_text:
        return ""
    
    # Tìm khối JavaScript
    js_match = re.search(r"```javascript(.*?)```", response_text, re.DOTALL)
    if not js_match:
        js_match = re.search(r"```js(.*?)```", response_text, re.DOTALL)
    
    if js_match:
        return js_match.group(1).strip()
    
    return ""


def _extract_css(response_text):
    """Trích xuất nội dung CSS từ phản hồi"""
    if not response_text:
        return ""
    
    # Tìm khối CSS
    css_match = re.search(r"```css(.*?)```", response_text, re.DOTALL)
    if css_match:
        return css_match.group(1).strip()
    
    return ""
