# app/services/workflow_nodes/translate_content.py

import time
from typing import Dict, Any
from google.genai import types
from .base import ReportState, read_prompt_file, get_prompt_from_env
from ...services.progress_tracker import progress_tracker


def translate_content_node(state: ReportState) -> Dict[str, Any]:
    """
    Node để dịch nội dung HTML và JavaScript từ tiếng Việt sang tiếng Anh bằng AI.
    
    Args:
        state: Trạng thái hiện tại của workflow
        
    Returns:
        Dict chứa nội dung đã dịch
    """
    session_id = state["session_id"]
    progress_tracker.update_step(session_id, 7, "Dịch nội dung", "Dịch HTML và JavaScript từ tiếng Việt sang tiếng Anh")
    
    try:
        print("\n=== BƯỚC DỊCH NỘI DUNG ===")
        print("Bắt đầu dịch HTML và JavaScript content từ tiếng Việt sang tiếng Anh...")
        
        translated_html = None
        translated_js = None
        
        # Dịch HTML content
        if state.get("html_content"):
            print("Đang dịch HTML content...")
            progress_tracker.update_step(session_id, details="Đang dịch HTML content...")
            translated_html = _translate_with_ai(
                state["client"], 
                state["model"], 
                state["html_content"], 
                "html",
                session_id
            )
            if translated_html:
                print("✓ HTML content đã được dịch thành công")
                progress_tracker.update_step(session_id, details=f"✓ HTML đã dịch - {len(translated_html)} chars")
            else:
                print("✗ Dịch HTML content thất bại")
        
        # Dịch JavaScript content
        if state.get("js_content"):
            print("Đang dịch JavaScript content...")
            progress_tracker.update_step(session_id, details="Đang dịch JavaScript content...")
            translated_js = _translate_with_ai(
                state["client"], 
                state["model"], 
                state["js_content"], 
                "javascript",
                session_id
            )
            if translated_js:
                print("✓ JavaScript content đã được dịch thành công")
                progress_tracker.update_step(session_id, details=f"✓ JavaScript đã dịch - {len(translated_js)} chars")
            else:
                print("✗ Dịch JavaScript content thất bại")
        else:
            translated_js = None
        
        # Cập nhật state với nội dung đã dịch và trả về state để tiếp tục workflow
        translated_count = 0
        
        if translated_html:
            state["html_content_en"] = translated_html
            del translated_html  # 🧹 Cleanup immediately after saving
            translated_count += 1
        else:
            # đảm bảo key tồn tại
            state.setdefault("html_content_en", None)

        if translated_js:
            state["js_content_en"] = translated_js
            del translated_js  # 🧹 Cleanup immediately after saving
            translated_count += 1
        else:
            state.setdefault("js_content_en", None)

        print(f"Translation node hoàn thành. Đã dịch {translated_count} nội dung.")
        progress_tracker.update_step(session_id, details=f"Hoàn thành dịch {translated_count} nội dung")
        
        # 🧹 Memory cleanup - force garbage collection
        import gc
        gc.collect()
        print("🧹 [translate_content] Memory cleanup completed")
        
        return state
        
    except Exception as e:
        error_msg = f"Translation node thất bại: {e}"
        print(f"ERROR: {error_msg}")
        progress_tracker.update_step(session_id, details=f"⚠️ Lỗi dịch: {e}")
    # Tiếp tục workflow ngay cả khi dịch thất bại - đảm bảo các khóa tồn tại trên state
    state.setdefault("html_content_en", None)
    state.setdefault("js_content_en", None)
    return state


def _translate_with_ai(client, model, content: str, content_type: str, session_id: str) -> str:
    """
    Dịch nội dung bằng AI.
    
    Args:
        client: Google GenAI client
        model: Model name
        content: Nội dung cần dịch
        content_type: Loại nội dung ("html" hoặc "javascript")
        session_id: Session ID cho progress tracking
        
    Returns:
        Nội dung đã dịch hoặc None nếu thất bại
    """
    if not content or len(content.strip()) == 0:
        return None
    
    # Tạo prompt dịch dựa trên loại content
    if content_type == "html":
        prompt_template = get_prompt_from_env('translate_html')
        if prompt_template is None:
            print("ERROR: Không thể đọc prompt_translate_html từ biến môi trường")
            return None
        prompt = prompt_template.replace('{content}', content)
    elif content_type == "javascript":
        prompt_template = get_prompt_from_env('translate_js')
        if prompt_template is None:
            print("ERROR: Không thể đọc prompt_translate_js từ biến môi trường")
            return None
        prompt = prompt_template.replace('{js_content}', content)
        # Nếu có HTML content trong state, có thể thêm vào
        # prompt = prompt.replace('{html_content}', state.get('html_content', ''))
    else:
        print(f"ERROR: Loại content không được hỗ trợ: {content_type}")
        return None
    
    # Tạo request cho AI
    contents = [
        types.Content(
            role="user",
            parts=[
                types.Part.from_text(text=prompt),
            ],
        ),
    ]
    
    config = types.GenerateContentConfig(
        temperature=0.1,  # Low temperature để dịch chính xác
        candidate_count=1,
    )
    
    # Retry logic giống như các node khác
    for attempt in range(3):
        try:
            progress_tracker.update_step(session_id, details=f"Gọi AI dịch {content_type} (lần {attempt + 1}/3)...")
            response = client.models.generate_content(
                model=model,
                contents=contents,
                config=config
            )
            
            if response and hasattr(response, 'text') and response.text:
                # Làm sạch response text
                translated_content = response.text.strip()
                
                # Loại bỏ markdown code blocks nếu có
                if translated_content.startswith('```'):
                    lines = translated_content.split('\n')
                    if len(lines) > 2:
                        # Bỏ dòng đầu và cuối (markdown markers)
                        translated_content = '\n'.join(lines[1:-1])
                
                # Kiểm tra nếu nội dung có thực sự có ý nghĩa
                if translated_content and len(translated_content.strip()) > 0:
                    result = translated_content
                    # 🧹 Cleanup response object trước khi return
                    del response
                    del translated_content
                    return result
                else:
                    print(f"WARNING: AI trả về nội dung rỗng cho {content_type}, thử lại...")
                    # 🧹 Cleanup trước khi retry
                    del response
                    del translated_content
                    if attempt < 2:
                        continue  # Thử lại trong vòng lặp
                    else:
                        return None
            else:
                print(f"WARNING: AI không trả về nội dung cho {content_type}, thử lại...")
                if attempt < 2:
                    continue  # Thử lại trong vòng lặp
                else:
                    return None
                
        except Exception as e:
            if attempt < 2:
                wait_time = (attempt + 1) * 10
                progress_tracker.update_step(session_id, details=f"Lỗi dịch {content_type}, chờ {wait_time}s...")
                print(f"WARNING: Lỗi dịch {content_type} (lần {attempt + 1}), thử lại sau {wait_time}s: {e}")
                time.sleep(wait_time)
            else:
                print(f"ERROR: Không thể dịch {content_type} sau 3 lần thử: {e}")
                return None
    
    return None
