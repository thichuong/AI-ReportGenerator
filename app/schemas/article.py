"""
Pydantic schemas cho bài viết
"""
from pydantic import BaseModel, Field
from typing import Optional
from datetime import datetime

class ArticleBase(BaseModel):
    """Base schema cho Article"""
    title: str = Field(..., min_length=1, max_length=255, description="Tiêu đề bài viết")
    content: Optional[str] = Field(None, description="Nội dung bài viết")
    summary: Optional[str] = Field(None, description="Tóm tắt bài viết")
    symbol: Optional[str] = Field(None, max_length=10, description="Mã cổ phiếu")
    report_type: Optional[str] = Field(None, max_length=50, description="Loại báo cáo")

class ArticleCreate(ArticleBase):
    """Schema để tạo bài viết mới"""
    pass

class ArticleUpdate(BaseModel):
    """Schema để cập nhật bài viết"""
    title: Optional[str] = Field(None, min_length=1, max_length=255)
    content: Optional[str] = None
    summary: Optional[str] = None
    symbol: Optional[str] = Field(None, max_length=10)
    report_type: Optional[str] = Field(None, max_length=50)
    is_published: Optional[bool] = None

class Article(ArticleBase):
    """Schema đầy đủ cho Article"""
    id: int
    is_published: bool = False
    created_at: datetime
    updated_at: Optional[datetime] = None
    
    class Config:
        from_attributes = True
