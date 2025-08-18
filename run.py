"""
File để chạy ứng dụng FastAPI
"""
import uvicorn

if __name__ == "__main__":
    uvicorn.run(
        "main:app", 
        host="0.0.0.0", 
        port=8888, 
        reload=True
    )
