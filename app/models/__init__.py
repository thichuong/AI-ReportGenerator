"""
Chứa các model SQLAlchemy
"""
from sqlalchemy import Column, Integer, String, Text, DateTime, Boolean
from sqlalchemy.sql import func
from datetime import datetime, timezone
from app.db.session import Base

class Article(Base):
    """
    Model cho bài viết/báo cáo đầu tư
    """
    __tablename__ = "articles"
    
    id = Column(Integer, primary_key=True, index=True)
    title = Column(String(255), nullable=False, index=True)
    content = Column(Text, nullable=True)
    summary = Column(Text, nullable=True)
    symbol = Column(String(10), nullable=True, index=True)  # Mã cổ phiếu
    report_type = Column(String(50), nullable=True)  # Loại báo cáo
    is_published = Column(Boolean, default=False)
    created_at = Column(DateTime(timezone=True), server_default=func.now())
    updated_at = Column(DateTime(timezone=True), onupdate=func.now())
    
    def __repr__(self):
        return f"<Article(id={self.id}, title='{self.title}', symbol='{self.symbol}')>"

class CryptoReport(Base):
    """
    Model để lưu trữ nội dung báo cáo được tạo ra bởi AI.
    Tương thích với model từ Crypto-Dashboard-and-AI-ReportGenerator
    """
    __tablename__ = 'crypto_report'
    
    id = Column(Integer, primary_key=True, index=True)
    html_content = Column(Text, nullable=False)
    css_content = Column(Text, nullable=True)
    js_content = Column(Text, nullable=True)
    html_content_en = Column(Text, nullable=True)  # Nội dung HTML đã dịch sang tiếng Anh
    js_content_en = Column(Text, nullable=True)    # Nội dung JS đã dịch sang tiếng Anh
    # Store timezone-aware UTC timestamps (maps to TIMESTAMPTZ in Postgres)
    created_at = Column(
        DateTime(timezone=True), 
        nullable=False, 
        default=lambda: datetime.now(timezone.utc)
    )
    
    def __repr__(self):
        return f'<CryptoReport {self.id}>'
