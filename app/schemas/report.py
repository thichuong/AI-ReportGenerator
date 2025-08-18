"""
Pydantic schemas cho báo cáo AI
"""
from pydantic import BaseModel, Field
from typing import Optional
from datetime import datetime

class ReportBase(BaseModel):
    """Base schema cho CryptoReport"""
    html_content: str = Field(..., description="Nội dung HTML báo cáo (bắt buộc)")
    css_content: Optional[str] = Field(None, description="CSS content")
    js_content: Optional[str] = Field(None, description="JavaScript content")
    html_content_en: Optional[str] = Field(None, description="Nội dung HTML tiếng Anh")
    js_content_en: Optional[str] = Field(None, description="JavaScript content tiếng Anh")

class ReportCreate(ReportBase):
    """Schema để tạo báo cáo mới"""
    html_content: str  # Bắt buộc khi tạo mới

class ReportUpdate(BaseModel):
    """Schema để cập nhật báo cáo"""
    html_content: Optional[str] = None
    css_content: Optional[str] = None
    js_content: Optional[str] = None
    html_content_en: Optional[str] = None
    js_content_en: Optional[str] = None

class Report(ReportBase):
    """Schema đầy đủ cho CryptoReport"""
    id: int
    created_at: datetime
    
    class Config:
        from_attributes = True

class ProgressResponse(BaseModel):
    """Schema cho response tiến độ"""
    session_id: str
    status: str
    progress: int
    message: str
    details: Optional[dict] = None
