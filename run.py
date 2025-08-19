"""
File để chạy ứng dụng FastAPI với Railway support
"""
import uvicorn
import os
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()

if __name__ == "__main__":
    # Railway configuration - dynamic port and host
    host = os.environ.get('HOST', '0.0.0.0')
    port = int(os.environ.get('PORT', 8888))
    
    # Enable debug mode only for local development
    debug_mode = os.environ.get('RAILWAY_ENVIRONMENT') != 'production'
    
    print(f"🚀 Starting AI Report Generator on {host}:{port}")
    print(f"🌍 Environment: {'production' if not debug_mode else 'development'}")
    
    # Run FastAPI with uvicorn
    uvicorn.run(
        "main:app",
        host=host,
        port=port,
        reload=debug_mode,  # Auto-reload for development
        access_log=True,
        log_level="info"
    )
