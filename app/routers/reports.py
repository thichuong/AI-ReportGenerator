"""
APIRouter cho các tính năng về báo cáo AI
"""
import os
import uuid
from fastapi import APIRouter, HTTPException, BackgroundTasks
from fastapi.responses import JSONResponse
from dotenv import load_dotenv

# Load environment variables
load_dotenv()

router = APIRouter()

@router.get("/check-api-key")
async def check_api_key():
    """
    Debug endpoint để kiểm tra API key
    """
    api_key = os.getenv('GEMINI_API_KEY')
    return {
        "has_api_key": bool(api_key),
        "api_key_length": len(api_key) if api_key else 0,
        "api_key_preview": f"{api_key[:10]}..." if api_key else None
    }

@router.post("/generate-auto-report")
async def generate_auto_report(
    background_tasks: BackgroundTasks
):
    """
    Tạo báo cáo tự động với AI workflow V2
    """
    try:
        api_key = os.getenv('GEMINI_API_KEY')
        
        if not api_key:
            raise HTTPException(
                status_code=400, 
                detail='Vui lòng cung cấp API Key hoặc thiết lập GEMINI_API_KEY.'
            )
        
        # Tạo session_id mới cho tracking
        session_id = str(uuid.uuid4())
        
        # Import và chạy workflow V2 trong background task
        from app.services.report_workflow_v2 import generate_auto_research_report_langgraph_v2
        from app.services.progress_tracker import progress_tracker
        
        def run_workflow_background():
            """Chạy workflow V2 trong background với error handling"""
            try:
                result = generate_auto_research_report_langgraph_v2(api_key, session_id=session_id)
                #print(f"Workflow V2 completed: {result}")
                print(f"Workflow V2 completed successfully for session {session_id}")
            except Exception as e:
                print(f"Workflow error: {e}")
                progress_tracker.error_progress(session_id, f"Lỗi workflow: {e}")
        
        # Khởi tạo progress tracking
        progress_tracker.start_progress(session_id)
        
        # Chạy workflow trong background task
        background_tasks.add_task(run_workflow_background)
        
        return JSONResponse({
            'success': True, 
            'message': 'Đã bắt đầu tạo báo cáo, theo dõi tiến độ qua API',
            'session_id': session_id
        })
            
    except Exception as e:
        raise HTTPException(status_code=500, detail=f'Đã xảy ra lỗi không mong muốn: {e}')

@router.get("/progress/{session_id}")
async def get_progress(session_id: str):
    """
    Lấy tiến độ tạo báo cáo theo session_id
    """
    try:
        from app.services.progress_tracker import progress_tracker
        progress_data = progress_tracker.get_progress(session_id)
        
        if not progress_data:
            raise HTTPException(status_code=404, detail='Session not found')
            
        return JSONResponse(progress_data)
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
