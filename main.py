"""
Điểm khởi chạy ứng dụng FastAPI
"""
from fastapi import FastAPI
from dotenv import load_dotenv
from app.routers import articles, reports
from app.db.session import create_tables
from app.services.auto_report_scheduler import start_auto_report_scheduler

# Load environment variables
load_dotenv()

# Tạo instance FastAPI
app = FastAPI(
    title="AI Report Generator API",
    description="API for generating investment reports using AI",
    version="1.0.0"
)

# Tạo database tables khi khởi động
create_tables()

# Khởi động auto report scheduler
@app.on_event("startup")
async def startup_event():
    """Initialize services when app starts"""
    print("🚀 Starting AI Report Generator...")
    
    # Start auto report scheduler if enabled
    scheduler_started = start_auto_report_scheduler()
    if scheduler_started:
        print("✅ Auto report scheduler started successfully")
    else:
        print("ℹ️ Auto report scheduler not started (check environment variables)")

# Đăng ký router
app.include_router(articles.router, prefix="/api/v1", tags=["articles"])
app.include_router(reports.router, prefix="/api/v1", tags=["reports"])

@app.get("/")
async def root():
    """Root endpoint"""
    return {"message": "Welcome to AI Report Generator API"}

@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {"status": "healthy"}
