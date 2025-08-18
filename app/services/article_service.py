"""
Chứa logic nghiệp vụ cho articles
"""
from sqlalchemy.orm import Session
from typing import List, Optional

from app.models import Article
from app.schemas.article import ArticleCreate, ArticleUpdate

class ArticleService:
    """Service class cho các thao tác với Article"""
    
    def __init__(self, db: Session):
        self.db = db
    
    def create_article(self, article_data: ArticleCreate) -> Article:
        """
        Tạo bài viết mới
        """
        db_article = Article(**article_data.dict())
        self.db.add(db_article)
        self.db.commit()
        self.db.refresh(db_article)
        return db_article
    
    def get_article_by_id(self, article_id: int) -> Optional[Article]:
        """
        Lấy bài viết theo ID
        """
        return self.db.query(Article).filter(Article.id == article_id).first()
    
    def get_articles(
        self, 
        skip: int = 0, 
        limit: int = 100,
        symbol: Optional[str] = None,
        report_type: Optional[str] = None
    ) -> List[Article]:
        """
        Lấy danh sách bài viết với phân trang và lọc
        """
        query = self.db.query(Article)
        
        if symbol:
            query = query.filter(Article.symbol == symbol)
        
        if report_type:
            query = query.filter(Article.report_type == report_type)
        
        return query.offset(skip).limit(limit).all()
    
    def update_article(self, article_id: int, article_update: ArticleUpdate) -> Optional[Article]:
        """
        Cập nhật bài viết
        """
        db_article = self.get_article_by_id(article_id)
        if db_article is None:
            return None
        
        update_data = article_update.dict(exclude_unset=True)
        for field, value in update_data.items():
            setattr(db_article, field, value)
        
        self.db.commit()
        self.db.refresh(db_article)
        return db_article
    
    def delete_article(self, article_id: int) -> bool:
        """
        Xóa bài viết
        """
        db_article = self.get_article_by_id(article_id)
        if db_article is None:
            return False
        
        self.db.delete(db_article)
        self.db.commit()
        return True
    
    def publish_article(self, article_id: int) -> Optional[Article]:
        """
        Xuất bản bài viết
        """
        db_article = self.get_article_by_id(article_id)
        if db_article is None:
            return None
        
        db_article.is_published = True
        self.db.commit()
        self.db.refresh(db_article)
        return db_article
    
    def get_published_articles(self, skip: int = 0, limit: int = 100) -> List[Article]:
        """
        Lấy danh sách bài viết đã xuất bản
        """
        return self.db.query(Article).filter(
            Article.is_published == True
        ).offset(skip).limit(limit).all()
    
    def search_articles_by_title(self, search_term: str) -> List[Article]:
        """
        Tìm kiếm bài viết theo tiêu đề
        """
        return self.db.query(Article).filter(
            Article.title.contains(search_term)
        ).all()
