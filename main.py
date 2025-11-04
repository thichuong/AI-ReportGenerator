"""
Điểm khởi chạy ứng dụng FastAPI
"""
from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from fastapi.responses import FileResponse
from dotenv import load_dotenv
from app.routers import articles, reports, market
from app.db.session import create_tables
from app.services.auto_report_scheduler import start_auto_report_scheduler
from app.utils.prompt_env_loader import load_prompt_envs
from apscheduler.schedulers.asyncio import AsyncIOScheduler
from apscheduler.triggers.interval import IntervalTrigger
import asyncio
import os

# Load environment variables
load_dotenv()

# Load all prompt environment variables from prompt_envs directory
load_prompt_envs()

# Tạo instance FastAPI
app = FastAPI(
    title="AI Report Generator API",
    description="API for generating investment reports using AI",
    version="1.0.0"
)

# Khởi tạo scheduler
scheduler = AsyncIOScheduler()

# Khởi động auto report scheduler và tạo database tables khi app bắt đầu
@app.on_event("startup")
async def startup_event():
    """Initialize services when app starts"""
    print("🚀 Starting AI Report Generator...")

    # Tạo database tables khi khởi động (bọc try/except để tránh crash lúc import)
    try:
        create_tables()
        print("✅ Database tables ensured/created")
    except Exception as e:
        # Log lỗi nhưng không ngăn server khởi động - health endpoint vẫn có thể phản hồi
        print(f"❌ Failed to create/ensure database tables: {e}")

    # Start auto report scheduler if enabled
    try:
        scheduler_started = start_auto_report_scheduler()
        if scheduler_started:
            print("✅ Auto report scheduler started successfully")
        else:
            print("ℹ️ Auto report scheduler not started (check environment variables)")
    except Exception as e:
        print(f"❌ Error while starting auto report scheduler: {e}")

    # Market indices scheduler removed - not used in this project. If you need
    # this functionality in the future, reintroduce the scheduling block here.

# Shutdown scheduler when app stops
@app.on_event("shutdown")
async def shutdown_event():
    """Shutdown services when app stops"""
    if scheduler.running:
        scheduler.shutdown()
        print("✅ Scheduler shut down")

# Đăng ký router
app.include_router(articles.router, prefix="/api/v1", tags=["articles"])
app.include_router(reports.router, prefix="/api/v1", tags=["reports"])
app.include_router(market.router, prefix="/api/v1", tags=["market"])

# Mount static files
static_path = os.path.join(os.path.dirname(__file__), "app", "static")
app.mount("/static", StaticFiles(directory=static_path), name="static")

@app.get("/")
async def root():
    """Root endpoint - serve homepage"""
    return FileResponse(os.path.join(static_path, "index.html"))

@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {"status": "healthy"}
