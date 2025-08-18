"""
APIRouter cho các tính năng về bài viết
"""
from fastapi import APIRouter, Depends, HTTPException, status
from sqlalchemy.orm import Session
from typing import List, Optional

from app.db.session import get_db
from app.schemas.article import Article, ArticleCreate, ArticleUpdate
from app.services.article_service import ArticleService

router = APIRouter()

@router.post("/articles/", response_model=Article, status_code=status.HTTP_201_CREATED)
async def create_article(
    article: ArticleCreate,
    db: Session = Depends(get_db)
):
    """
    Tạo bài viết mới
    """
    article_service = ArticleService(db)
    return article_service.create_article(article)

@router.get("/articles/", response_model=List[Article])
async def get_articles(
    skip: int = 0,
    limit: int = 100,
    symbol: Optional[str] = None,
    report_type: Optional[str] = None,
    db: Session = Depends(get_db)
):
    """
    Lấy danh sách bài viết
    """
    article_service = ArticleService(db)
    return article_service.get_articles(
        skip=skip, 
        limit=limit, 
        symbol=symbol, 
        report_type=report_type
    )

@router.get("/articles/{article_id}", response_model=Article)
async def get_article(
    article_id: int,
    db: Session = Depends(get_db)
):
    """
    Lấy thông tin chi tiết bài viết
    """
    article_service = ArticleService(db)
    article = article_service.get_article_by_id(article_id)
    if article is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Article not found"
        )
    return article

@router.put("/articles/{article_id}", response_model=Article)
async def update_article(
    article_id: int,
    article_update: ArticleUpdate,
    db: Session = Depends(get_db)
):
    """
    Cập nhật bài viết
    """
    article_service = ArticleService(db)
    article = article_service.update_article(article_id, article_update)
    if article is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Article not found"
        )
    return article

@router.delete("/articles/{article_id}", status_code=status.HTTP_204_NO_CONTENT)
async def delete_article(
    article_id: int,
    db: Session = Depends(get_db)
):
    """
    Xóa bài viết
    """
    article_service = ArticleService(db)
    success = article_service.delete_article(article_id)
    if not success:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Article not found"
        )

@router.post("/articles/{article_id}/publish")
async def publish_article(
    article_id: int,
    db: Session = Depends(get_db)
):
    """
    Xuất bản bài viết
    """
    article_service = ArticleService(db)
    article = article_service.publish_article(article_id)
    if article is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Article not found"
        )
    return {"message": "Article published successfully", "article": article}
