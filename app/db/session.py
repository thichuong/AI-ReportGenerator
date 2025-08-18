"""
Logic quản lý database session
"""
from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker
import os
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()

# Database URL - ưu tiên PostgreSQL từ .env
DATABASE_URL = os.getenv("DATABASE_URL", "sqlite:///./ai_report.db")

# Tạo engine với cấu hình tối ưu cho PostgreSQL
if "postgresql" in DATABASE_URL:
    # Cấu hình PostgreSQL
    engine = create_engine(
        DATABASE_URL,
        pool_size=10,
        max_overflow=20,
        pool_pre_ping=True,  # Kiểm tra kết nối trước khi sử dụng
        echo=False  # Set True để debug SQL queries
    )
else:
    # Cấu hình SQLite (fallback)
    engine = create_engine(
        DATABASE_URL, 
        connect_args={"check_same_thread": False}
    )

# Tạo SessionLocal
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)

# Base class cho các model
Base = declarative_base()

def get_db():
    """
    Dependency để lấy database session
    """
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()

def create_tables():
    """
    Tạo tất cả các bảng trong database
    """
    Base.metadata.create_all(bind=engine)
