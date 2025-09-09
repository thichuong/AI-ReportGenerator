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
    Tạo báo cáo tự động với AI workflow
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
        
        # Import và chạy workflow trong background task
        from app.services.report_workflow import generate_auto_research_report_langgraph
        from app.services.progress_tracker import progress_tracker
        
        def run_workflow_background():
            """Chạy workflow trong background với error handling"""
            try:
                result = generate_auto_research_report_langgraph(api_key, session_id=session_id)
                #print(f"Workflow completed: {result}")
                print(f"Workflow completed successfully for session {session_id}")
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


@router.post("/manual-generate")
async def generate_manual_report():
    """
    Tạo báo cáo thủ công ngay lập tức (không dùng background task)
    """
    try:
        from app.services.auto_report_scheduler import create_manual_report
        
        result = create_manual_report()
        
        if result['success']:
            return JSONResponse({
                'success': True,
                'message': result['message'],
                'report_id': result['report_id'],
                'duration': result['duration']
            })
        else:
            raise HTTPException(
                status_code=500, 
                detail=result['error']
            )
            
    except Exception as e:
        raise HTTPException(status_code=500, detail=f'Lỗi khi tạo báo cáo thủ công: {e}')


@router.get("/scheduler/status")
async def scheduler_status():
    """
    Kiểm tra trạng thái auto report scheduler
    """
    import os
    
    return JSONResponse({
        'scheduler_enabled': os.getenv('ENABLE_AUTO_REPORT_SCHEDULER', 'false').lower() == 'true',
        'api_key_configured': bool(os.getenv('GEMINI_API_KEY')),
        'interval_hours': int(os.getenv('AUTO_REPORT_INTERVAL_HOURS', '3')),
        'max_attempts': int(os.getenv('MAX_REPORT_ATTEMPTS', '3'))
    })
