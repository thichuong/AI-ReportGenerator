# AI Report Generator

## Mô tả
Ứng dụng AI để tạo báo cáo đầu tư tự động với FastAPI backend.

## Cấu trúc dự án
```
AI-ReportGenerator/
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
├── create_report/            # Module tạo báo cáo AI
├── main.py                   # FastAPI application instance
├── run.py                    # Điểm khởi chạy ứng dụng
├── requirements.txt          # Dependencies
├── Dockerfile                # File cấu hình Docker
├── docker-compose.yml        # Docker Compose configuration
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

### 4. Docker Deployment (Khuyến nghị)

#### Quick Deploy
```bash
# Chạy script tự động deploy
./deploy.sh
```

#### Manual Docker Deploy
```bash
# Tạo thư mục cần thiết
mkdir -p data logs

# Build và chạy với Docker Compose
docker-compose up -d --build

# Kiểm tra logs
docker-compose logs -f

# Dừng services
docker-compose down
```

#### Railway Deployment
```bash
# Cài đặt Railway CLI
npm install -g @railway/cli

# Login và deploy
railway login
railway up
```

### 5. Development Mode
```bash
# Chạy development server
python run.py

# Hoặc với uvicorn
uvicorn main:app --reload
```

## API Endpoints

### Articles
- `POST /api/v1/articles/` - Tạo bài viết mới
- `GET /api/v1/articles/` - Lấy danh sách bài viết
- `GET /api/v1/articles/{id}` - Lấy chi tiết bài viết
- `PUT /api/v1/articles/{id}` - Cập nhật bài viết
- `DELETE /api/v1/articles/{id}` - Xóa bài viết
- `POST /api/v1/articles/{id}/publish` - Xuất bản bài viết

### AI Reports
- `POST /api/v1/generate-auto-report` - Tạo báo cáo tự động bằng AI
- `GET /api/v1/progress/{session_id}` - Theo dõi tiến độ tạo báo cáo
- `GET /api/v1/check-api-key` - Kiểm tra API key (debug)

### System & Monitoring
- `GET /` - Root endpoint
- `GET /health` - Health check endpoint
- `GET /docs` - Swagger documentation
- `GET /redoc` - ReDoc documentation

## 🔍 Monitoring & Health Checks

### Local Development
```bash
# Health check
curl http://localhost:8888/health

# API documentation
curl http://localhost:8888/docs
```

### Docker Deployment
```bash
# Container status
docker-compose ps

# Health check
curl http://localhost:8888/health

# View logs
docker-compose logs -f ai-report-generator

# Container stats
docker stats
```

## Tính năng

- ✅ FastAPI REST API
- ✅ CRUD operations cho articles
- ✅ AI-powered auto report generation với Gemini AI
- ✅ LangGraph workflow cho tự động hóa
- ✅ Progress tracking cho long-running tasks
- ✅ Background task processing
- ✅ Pydantic schemas cho validation
- ✅ SQLAlchemy models
- ✅ Service layer pattern
- ✅ Database session management
- ✅ Docker & Docker Compose support
- ✅ Environment configuration
- ✅ API documentation với Swagger/ReDoc

## Công nghệ sử dụng

- **FastAPI** - Web framework
- **SQLAlchemy** - ORM
- **Pydantic** - Data validation
- **Uvicorn** - ASGI server
- **SQLite** - Database (có thể thay đổi)
- **Google Gemini AI** - AI content generation
- **LangChain & LangGraph** - AI workflow orchestration
- **Beautiful Soup** - HTML parsing
- **Requests** - HTTP client

## Truy cập API Documentation

Sau khi chạy ứng dụng, bạn có thể truy cập:
- **Swagger UI:** `http://localhost:8000/docs`
- **ReDoc:** `http://localhost:8000/redoc`

## License

Dự án này được phát hành dưới **Apache License 2.0**. Xem file [LICENSE](LICENSE) để biết chi tiết.

## Rust Google VM Deployment (CI/CD)

Dự án có workflow tự động build và deploy ứng dụng Rust lên Google VM.

### Cấu hình GitHub Secrets

Để workflow hoạt động, bạn cần cấu hình các secrets sau trong phần **Settings > Secrets and variables > Actions** của repository:

- `SSH_HOST`: Địa chỉ IP public của Google VM.
- `SSH_USER`: Username SSH (ví dụ: `thichuong`).
- `SSH_KEY`: SSH private key để truy cập VM.

### Workflow hoạt động như thế nào?

1.  Mỗi khi có commit lên nhánh `main`, workflow `.github/workflows/rust-cd.yml` sẽ được kích hoạt.
2.  Mã nguồn Rust sẽ được build với profile `--release`.
3.  File binary `ai-report-generator` sẽ được copy vào thư mục `~/AI-ReportGenerator/` trên máy chủ Google VM.
4.  Ứng dụng sẽ được **khởi động lại tự động** (stop process cũ, start process mới chạy nền).
5.  Logs sẽ được lưu tại `~/AI-ReportGenerator/app.log`.