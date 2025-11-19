"""
Node thực hiện nghiên cứu sâu + validation
"""
import json
from google.genai import types
from .base import ReportState, check_report_validation, call_gemini_with_rate_limit_handling
from ...services.progress_tracker import progress_tracker


def research_deep_node(state: ReportState) -> ReportState:
    """Node để thực hiện nghiên cứu sâu + validation với Google Search và real-time data trong 1 lần gọi"""
    session_id = state["session_id"]

    # CHECK RATE LIMIT FLAG - Skip node if already hit rate limit
    if state.get("rate_limit_stop"):
        print(f"⛔ [{session_id}] Skipping research_deep - rate limit flag is set")
        return state

    # Initialize current_attempt if not exists
    if "current_attempt" not in state:
        state["current_attempt"] = 0
    state["current_attempt"] += 1
    
    progress_tracker.update_step(session_id, 2, f"Research + Validation (lần {state['current_attempt']})", 
                               "Cấu hình AI tools, Google Search và thực hiện combined research + validation")
    
    try:
        # Chuẩn bị combined prompt với real-time data
        combined_prompt = state["research_analysis_prompt"]
        
        # Thêm real-time data vào prompt
        realtime_data = state.get("realtime_data")
        if realtime_data:
            # Inject real-time data vào combined prompt
            combined_prompt = combined_prompt.replace(
                "{{REAL_TIME_DATA}}", 
                json.dumps(realtime_data, ensure_ascii=False, indent=2)
            )
            progress_tracker.update_step(session_id, details="✓ Đã inject real-time data vào combined prompt")
        else:
            # Thay thế bằng fallback message
            combined_prompt = combined_prompt.replace(
                "{{REAL_TIME_DATA}}", 
                "{\n  \"notice\": \"Real-time data không khả dụng, sử dụng Google Search để lấy dữ liệu mới nhất\"\n}"
            )
            progress_tracker.update_step(session_id, details="⚠️ Không có real-time data, sử dụng Google Search")
        
        # Cấu hình tools với thinking budget cao hơn cho combined task
        tools = [
            types.Tool(googleSearch=types.GoogleSearch()),
        ]
        generate_content_config = types.GenerateContentConfig(
            thinking_config=types.ThinkingConfig(
                thinking_budget=8192,  # Giảm thinking xuống ~8k-10k để dành đất cho nội dung
            ),
            tools=tools,
            temperature=0.7,
            candidate_count=1,
            max_output_tokens=60000,
        )
        
        # Tạo request content với combined prompt
        contents = [
            types.Content(
                role="user",
                parts=[
                    types.Part.from_text(text=combined_prompt),
                ],
            ),
        ]
        
        # Gọi API 3 lần để có 3 response khác nhau (do model không hỗ trợ multiple candidates)
        all_responses = []

        for call_attempt in range(1):
            progress_tracker.update_step(session_id, details=f"Gọi Combined AI API lần {call_attempt + 1}/3...")

            # Use centralized error handler
            response, error_msg, is_rate_limit = call_gemini_with_rate_limit_handling(
                client=state["client"],
                model=state["model"],
                contents=contents,
                config=generate_content_config,
                session_id=session_id,
                operation_name=f"research_deep_call_{call_attempt + 1}",
                max_retries=3
            )

            # Check for rate limit error - stop immediately
            if is_rate_limit:
                state["error_messages"].append(error_msg)
                state["success"] = False
                state["rate_limit_stop"] = True  # SET FLAG to stop workflow
                progress_tracker.update_step(session_id, details=f"🚫 Rate limit error - đã set flag dừng workflow")
                print(f"⛔ [{session_id}] rate_limit_stop flag SET - workflow will terminate")
                return state

            # Check for other errors after retries
            if error_msg:
                progress_tracker.update_step(session_id, details=f"Lỗi API call {call_attempt + 1} sau 3 lần thử: {error_msg}")
                response = None

            # Kiểm tra và lưu response
            if response and hasattr(response, 'text') and response.text:
                all_responses.append(f"=== RESPONSE {call_attempt + 1} ===\n{response.text}\n")
                progress_tracker.update_step(session_id, details=f"✓ Thành công API call {call_attempt + 1}/3")
            else:
                progress_tracker.update_step(session_id, details=f"✗ Không nhận được response hợp lệ từ call {call_attempt + 1}")
        
        # Kiểm tra có ít nhất 1 response thành công
        if not all_responses:
            error_msg = f"Lần thử {state['current_attempt']}: Không nhận được response hợp lệ từ bất kỳ API call nào"
            state["error_messages"].append(error_msg)
            progress_tracker.update_step(session_id, details=error_msg)
            state["success"] = False
            return state
        
        # Kết hợp tất cả responses
        full_response_text = "\n".join(all_responses)
        
        # Parse combined response để extract research content và validation result
        progress_tracker.update_step(session_id, details=f"Parsing combined response với {len(all_responses)} responses...")
        
        # Tìm validation result trong toàn bộ combined response
        validation_result = check_report_validation(full_response_text)
        state["validation_result"] = validation_result
        
        state["research_content"] = full_response_text
        
        # Set success based on validation result
        if validation_result == "PASS":
            state["success"] = True
            progress_tracker.update_step(session_id, details=f"✓ Combined Research + Validation PASS")
        elif validation_result == "FAIL":
            state["success"] = False
            progress_tracker.update_step(session_id, details=f"✗ Combined Research + Validation FAIL")
        else:
            # UNKNOWN validation result - treat as success but log warning
            state["success"] = True
            state["validation_result"] = "UNKNOWN"
            progress_tracker.update_step(session_id, details=f"? Combined Response với validation UNKNOWN")
        
        # Log response length for debugging
        progress_tracker.update_step(session_id, details=
            f"✓ Combined response: {len(full_response_text)} chars từ {len(all_responses)} responses, "
            f"validation: {validation_result}")
        
        # 🧹 Memory cleanup - giải phóng temporary large objects
        del all_responses  # Xóa list chứa 3 response texts lớn
        del full_response_text  # Xóa combined text (đã lưu vào state["research_content"])
        import gc
        gc.collect()
        print("🧹 [research_deep] Memory cleanup completed")
        
    except Exception as e:
        error_msg = f"Lần thử {state['current_attempt']}: Lỗi khi gọi Combined AI: {e}"
        state["error_messages"].append(error_msg)
        progress_tracker.update_step(session_id, details=error_msg)
        state["success"] = False
        
        # 🧹 Memory cleanup ngay cả khi có lỗi
        import gc
        gc.collect()
    
    return state
