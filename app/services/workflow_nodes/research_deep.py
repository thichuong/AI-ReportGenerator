"""
Node thực hiện nghiên cứu sâu + validation
"""
import time
import json
from google.genai import types
from .base import ReportState, check_report_validation
from ...services.progress_tracker import progress_tracker


def research_deep_node(state: ReportState) -> ReportState:
    """Node để thực hiện nghiên cứu sâu + validation với Google Search và real-time data trong 1 lần gọi"""
    session_id = state["session_id"]
    
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
                thinking_budget=24576,  # Tăng thinking budget cho combined task
            ),
            tools=tools,
            temperature=0.5,
            candidate_count=1,
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
        
        for call_attempt in range(3):
            progress_tracker.update_step(session_id, details=f"Gọi Combined AI API lần {call_attempt + 1}/3...")
            
            # Retry cho mỗi API call
            response = None
            for api_attempt in range(3):
                try:
                    response = state["client"].models.generate_content(
                        model=state["model"],
                        contents=contents,
                        config=generate_content_config
                    )
                    break
                except Exception as api_error:
                    if api_attempt < 2:
                        wait_time = (api_attempt + 1) * 30
                        progress_tracker.update_step(session_id, details=f"Lỗi API call {call_attempt + 1}, retry {api_attempt + 1}, chờ {wait_time}s... ({api_error})")
                        time.sleep(wait_time)
                    else:
                        # Nếu hết retry cho call này, log error nhưng tiếp tục với call tiếp theo
                        progress_tracker.update_step(session_id, details=f"Lỗi API call {call_attempt + 1} sau 3 lần thử: {api_error}")
                        response = None
                        break
            
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
        
    except Exception as e:
        error_msg = f"Lần thử {state['current_attempt']}: Lỗi khi gọi Combined AI: {e}"
        state["error_messages"].append(error_msg)
        progress_tracker.update_step(session_id, details=error_msg)
        state["success"] = False
    
    return state
