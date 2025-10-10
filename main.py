"""
Điểm khởi chạy ứng dụng FastAPI
"""
from fastapi import FastAPI
from dotenv import load_dotenv
from app.routers import articles, reports, market
from app.db.session import create_tables
from app.services.auto_report_scheduler import start_auto_report_scheduler
from app.utils.prompt_env_loader import load_prompt_envs
from apscheduler.schedulers.asyncio import AsyncIOScheduler
from apscheduler.triggers.interval import IntervalTrigger
import asyncio

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

    # Start market indices scheduler
    try:
        from app.routers.market import fetch_and_store_market_indices
        scheduler.add_job(
            fetch_and_store_market_indices,
            trigger=IntervalTrigger(minutes=3),
            id='fetch_market_indices',
            name='Fetch and store market indices every 3 minutes'
        )
        scheduler.start()
        print("✅ Market indices scheduler started (runs every 3 minutes)")
    except Exception as e:
        print(f"❌ Error while starting market indices scheduler: {e}")

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

@app.get("/")
async def root():
    """Root endpoint"""
    return {"message": "Welcome to AI Report Generator API"}

@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {"status": "healthy"}
