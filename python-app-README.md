# AI Report Generator FastAPI

## Mô tả
Ứng dụng FastAPI để tạo báo cáo đầu tư sử dụng AI.

## Cấu trúc dự án
```
python-app/
├── app/
│   ├── __init__.py
│   ├── db/
│   │   ├── __init__.py
│   │   └── session.py        # Logic quản lý database session
│   ├── models/
│   │   └── __init__.py       # Chứa các model SQLAlchemy
│   ├── routers/
│   │   ├── __init__.py
│   │   └── articles.py       # APIRouter cho các tính năng về bài viết
│   ├── schemas/
│   │   ├── __init__.py
│   │   └── article.py        # Pydantic schemas cho bài viết
│   └── services/
│       ├── __init__.py
│       └── article_service.py # Chứa logic nghiệp vụ
├── main.py                   # FastAPI application instance
├── run.py                    # Điểm khởi chạy ứng dụng
├── requirements.txt          # Dependencies
├── Dockerfile                # File cấu hình Docker
└── .env.example              # File cấu hình môi trường mẫu
```

## Cài đặt và chạy

### 1. Cài đặt dependencies
```bash
pip install -r requirements.txt
```

### 2. Cấu hình môi trường
```bash
cp .env.example .env
```

### 3. Chạy ứng dụng
```bash
python run.py
```

Hoặc sử dụng uvicorn trực tiếp:
```bash
uvicorn main:app --reload
```

### 4. Chạy với Docker
```bash
docker build -t ai-report-api .
docker run -p 8000:8000 ai-report-api
```

## API Endpoints

- `GET /` - Root endpoint
- `GET /health` - Health check
- `GET /docs` - Swagger documentation
- `POST /api/v1/articles/` - Tạo bài viết mới
- `GET /api/v1/articles/` - Lấy danh sách bài viết
- `GET /api/v1/articles/{id}` - Lấy chi tiết bài viết
- `PUT /api/v1/articles/{id}` - Cập nhật bài viết
- `DELETE /api/v1/articles/{id}` - Xóa bài viết
- `POST /api/v1/articles/{id}/publish` - Xuất bản bài viết

## Tính năng

- ✅ CRUD operations cho articles
- ✅ Pydantic schemas cho validation
- ✅ SQLAlchemy models
- ✅ Service layer pattern
- ✅ Database session management
- ✅ Docker support
- ✅ Environment configuration
- ✅ API documentation với Swagger

## Công nghệ sử dụng

- **FastAPI** - Web framework
- **SQLAlchemy** - ORM
- **Pydantic** - Data validation
- **Uvicorn** - ASGI server
- **SQLite** - Database (có thể thay đổi)
