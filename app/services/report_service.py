"""
Service class cho các thao tác với CryptoReport
"""
from sqlalchemy.orm import Session
from typing import List, Optional

from app.models import CryptoReport
from app.schemas.report import ReportCreate, ReportUpdate

class ReportService:
    """Service class cho các thao tác với CryptoReport"""
    
    def __init__(self, db: Session):
        self.db = db
    
    def create_report(self, report_data: ReportCreate) -> CryptoReport:
        """
        Tạo báo cáo mới
        """
        db_report = CryptoReport(**report_data.dict())
        self.db.add(db_report)
        self.db.commit()
        self.db.refresh(db_report)
        return db_report
    
    def get_report_by_id(self, report_id: int) -> Optional[CryptoReport]:
        """
        Lấy báo cáo theo ID
        """
        return self.db.query(CryptoReport).filter(CryptoReport.id == report_id).first()
    
    def get_reports(
        self, 
        skip: int = 0, 
        limit: int = 100
    ) -> List[CryptoReport]:
        """
        Lấy danh sách báo cáo với phân trang
        """
        return self.db.query(CryptoReport).offset(skip).limit(limit).all()
    
    def update_report(self, report_id: int, report_update: ReportUpdate) -> Optional[CryptoReport]:
        """
        Cập nhật báo cáo
        """
        db_report = self.get_report_by_id(report_id)
        if db_report is None:
            return None
        
        update_data = report_update.dict(exclude_unset=True)
        for field, value in update_data.items():
            setattr(db_report, field, value)
        
        self.db.commit()
        self.db.refresh(db_report)
        return db_report
    
    def delete_report(self, report_id: int) -> bool:
        """
        Xóa báo cáo
        """
        db_report = self.get_report_by_id(report_id)
        if db_report is None:
            return False
        
        self.db.delete(db_report)
        self.db.commit()
        return True
    
    def get_latest_report(self) -> Optional[CryptoReport]:
        """
        Lấy báo cáo mới nhất
        """
        return self.db.query(CryptoReport).order_by(
            CryptoReport.created_at.desc()
        ).first()
    
    def get_all_reports_ordered(self) -> List[CryptoReport]:
        """
        Lấy tất cả báo cáo theo thứ tự thời gian giảm dần
        """
        return self.db.query(CryptoReport).order_by(
            CryptoReport.created_at.desc()
        ).all()
