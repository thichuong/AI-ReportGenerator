"""
File để chạy ứng dụng FastAPI
"""
import uvicorn
import os

if __name__ == "__main__":
    # Log thông tin startup
    print("🚀 Starting AI Report Generator...")
    print(f"📍 Host: 0.0.0.0")
    print(f"🔌 Port: 8888")
    print(f"🌍 Environment: {os.getenv('DEBUG', 'False')}")
    
    uvicorn.run(
        "main:app", 
        host="0.0.0.0", 
        port=8888, 
        reload=False,  # Tắt reload trong Docker để stable hơn
        log_level="info"
    )
