# AI Report Generator - Tài liệu Kiến trúc

## 🏗️ System Architecture Overview

AI Report Generator là một ứng dụng web hiện đại được xây dựng trên **FastAPI** với **LangGraph** workflow orchestration, tích hợp AI để tự động tạo báo cáo cryptocurrency real-time.

## 🎯 High-Level Architecture

```mermaid
graph TB
    subgraph "Client Layer"
        A[Web Browser]
        B[API Clients]
        C[Mobile Apps]
    end
    
    subgraph "API Gateway Layer"
        D[FastAPI Application]
        E[Swagger UI]
        F[Static Files]
    end
    
    subgraph "Business Logic Layer"
        G[Report Router]
        H[Article Router]
        I[Background Tasks]
    end
    
    subgraph "Service Layer"
        J[Workflow Orchestrator]
        K[Progress Tracker]
        L[External API Services]
    end
    
    subgraph "AI Layer"
        M[LangGraph Workflow]
        N[Gemini AI Client]
        O[Prompt Engine]
    end
    
    subgraph "Data Layer"
        P[PostgreSQL Database]
        Q[SQLAlchemy ORM]
        R[Connection Pool]
    end
    
    subgraph "External Services"
        S[CoinGecko API]
        T[Alternative.me API]
        U[Google Gemini AI]
    end
    
    A --> D
    B --> D
    C --> D
    D --> G
    D --> H
    G --> I
    H --> I
    I --> J
    J --> K
    J --> L
    K --> M
    L --> S
    L --> T
    M --> N
    N --> U
    M --> O
    J --> Q
    Q --> P
    Q --> R
```

## 📁 Project Structure

```
AI-ReportGenerator/
├── app/                          # Main application package
│   ├── __init__.py              # Package initialization
│   ├── db/                      # Database layer
│   │   ├── __init__.py
│   │   └── session.py           # SQLAlchemy configuration
│   ├── models/                  # Data models
│   │   ├── __init__.py          # Model exports
│   │   ├── article.py           # Article model
│   │   └── crypto_report.py     # Report model
│   ├── routers/                 # API route handlers
│   │   ├── __init__.py
│   │   ├── articles.py          # Article CRUD endpoints
│   │   └── reports.py           # Report generation endpoints
│   ├── services/                # Business logic services
│   │   ├── __init__.py          # Service exports
│   │   ├── coingecko.py         # CoinGecko API client
│   │   ├── alternative_me.py    # Fear & Greed Index client
│   │   ├── progress_tracker.py  # Progress tracking service
│   │   ├── report_workflow_v2.py # Main workflow orchestrator
│   │   └── workflow_nodes/      # LangGraph workflow nodes
│   │       ├── __init__.py
│   │       ├── base.py          # Base types and utilities
│   │       ├── research_deep.py # Research analysis node
│   │       ├── validate_report.py # Data validation node
│   │       ├── generate_report.py # Report generation node
│   │       ├── create_html.py   # HTML creation node
│   │       ├── create_css.py    # CSS creation node
│   │       ├── create_javascript.py # JS creation node
│   │       ├── translate_content.py # Translation node
│   │       └── save_database.py # Database save node
│   └── static/                  # Static files
│       └── css/
│           └── styles.css
├── create_report/               # AI prompts and resources
│   ├── colors.css               # CSS variables
│   ├── prompt_research_analysis.md
│   ├── prompt_data_validation.md
│   ├── prompt_create_report.md
│   ├── prompt_create_html.md
│   ├── prompt_create_css.md
│   ├── prompt_create_javascript.md
│   └── prompt_translate_html.md
├── docs/                        # Documentation
│   ├── ARCHITECTURE.md          # This file
│   ├── WORKFLOW.md              # Workflow documentation
│   └── UPDATES.md               # Update history
├── requirements.txt             # Python dependencies
├── main.py                      # FastAPI application factory
├── run.py                       # Server startup script
├── Dockerfile                   # Container configuration
├── docker-compose.yml           # Multi-service setup
├── .env.example                # Environment template
└── README.md                    # Project overview
```

## 🔧 Technical Stack

### Core Technologies
- **Framework:** FastAPI 0.104.1+ (ASGI web framework)
- **AI Orchestration:** LangGraph 0.2.0+ (workflow management)
- **AI Model:** Google Gemini 1.5 Pro (text generation)
- **Database:** PostgreSQL + SQLAlchemy 2.0.23+ (ORM)
- **Async Runtime:** Uvicorn (ASGI server)
- **Data Validation:** Pydantic 2.5.0+ (type validation)

### Supporting Libraries
- **HTTP Client:** httpx (async HTTP requests)
- **Environment:** python-dotenv (configuration)
- **Database Driver:** psycopg2-binary (PostgreSQL)
- **Concurrent Processing:** concurrent.futures (threading)

### External APIs
- **CoinGecko:** Cryptocurrency market data
- **Alternative.me:** Fear & Greed Index
- **Google AI:** Gemini API for text generation

## 🏛️ Architectural Patterns

### 1. Layered Architecture
```
┌─────────────────────────────────┐
│         Presentation Layer      │  ← FastAPI Routers
├─────────────────────────────────┤
│         Business Logic Layer    │  ← Services & Workflow
├─────────────────────────────────┤
│         Data Access Layer       │  ← SQLAlchemy Models
├─────────────────────────────────┤
│         Infrastructure Layer    │  ← Database, External APIs
└─────────────────────────────────┘
```

### 2. Microservice-like Services
```python
# Service separation
app/services/
├── coingecko.py          # External API integration
├── alternative_me.py     # External API integration  
├── progress_tracker.py   # State management
└── report_workflow_v2.py # Orchestration
```

### 3. Repository Pattern (via SQLAlchemy)
```python
# Data access abstraction
class CryptoReport(Base):
    __tablename__ = "crypto_reports"
    # Model definition với business logic
    
# Service layer sử dụng models
def save_report(report_data):
    report = CryptoReport(**report_data)
    session.add(report)
    session.commit()
```

## 🔄 Component Interactions

### 1. Request Flow
```mermaid
sequenceDiagram
    participant Client
    participant FastAPI
    participant BackgroundTask
    participant Workflow
    participant AI
    participant Database
    
    Client->>FastAPI: POST /reports/auto-generate
    FastAPI->>BackgroundTask: Start workflow
    FastAPI-->>Client: Return session_id
    
    BackgroundTask->>Workflow: Initialize state
    Workflow->>AI: Generate research
    AI-->>Workflow: Research content
    Workflow->>AI: Validate data
    AI-->>Workflow: Validation result
    Workflow->>AI: Create HTML/CSS/JS
    AI-->>Workflow: Generated code
    Workflow->>Database: Save report
    Database-->>Workflow: Report ID
    
    loop Progress Check
        Client->>FastAPI: GET /reports/progress/{session_id}
        FastAPI-->>Client: Progress status
    end
```

### 2. Service Dependencies
```python
# Dependency injection pattern
def get_db():
    """Database session dependency"""
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()

# Router sử dụng dependency
@router.post("/auto-generate")
async def generate_report(
    background_tasks: BackgroundTasks,
    db: Session = Depends(get_db)
):
    # Implementation
```

### 3. State Management
```python
# Centralized state in workflow
class ReportState(TypedDict):
    session_id: str
    current_step: int
    progress_data: dict
    error_messages: List[str]
    
# State persistence
progress_tracker = ProgressTracker()
progress_tracker.update_step(session_id, details)
```

## 🗄️ Database Design

### Entity Relationship Diagram
```mermaid
erDiagram
    ARTICLES ||--o{ CRYPTO_REPORTS : references
    
    ARTICLES {
        int id PK
        string title
        text content
        datetime created_at
        datetime updated_at
    }
    
    CRYPTO_REPORTS {
        int id PK
        text html_content
        text css_content  
        text js_content
        text html_content_en
        text js_content_en
        datetime created_at
        datetime updated_at
    }
```

### Database Configuration
```python
# Multi-database support
if "postgresql" in DATABASE_URL:
    engine = create_engine(
        DATABASE_URL,
        pool_size=10,           # Connection pool
        max_overflow=20,        # Extra connections
        pool_pre_ping=True,     # Health check
        echo=False             # SQL logging
    )
else:
    # SQLite fallback
    engine = create_engine(
        DATABASE_URL,
        connect_args={"check_same_thread": False}
    )
```

### Migration Strategy
- **Auto-migration:** SQLAlchemy metadata tự động tạo tables
- **Schema evolution:** Backward compatible changes
- **Data integrity:** Foreign key constraints
- **Performance:** Indexed columns cho queries

## 🔀 Workflow Architecture

### LangGraph State Machine
```python
# Workflow definition
workflow = StateGraph(ReportState)

# Add nodes
workflow.add_node("research", research_deep_node)
workflow.add_node("validate", validate_report_node) 
workflow.add_node("generate", generate_report_node)
workflow.add_node("html", create_html_node)
workflow.add_node("css", create_css_node)
workflow.add_node("js", create_js_node)
workflow.add_node("translate", translate_content_node)
workflow.add_node("save", save_database_node)

# Define edges (control flow)
workflow.add_edge(START, "research")
workflow.add_edge("research", "validate")
workflow.add_conditional_edges("validate", should_continue)
workflow.add_edge("generate", "html")
# ... more edges
```

### Node Architecture
```python
# Base node interface
def workflow_node(state: ReportState) -> ReportState:
    """
    Standard node signature
    Input: Current state
    Output: Updated state
    """
    try:
        # Node logic
        result = process_step(state)
        state.update(result)
        return state
    except Exception as e:
        state["error_messages"].append(str(e))
        return state
```

## 🔧 Configuration Management

### Environment-Based Config
```python
# .env file structure
DATABASE_URL=postgresql://user:pass@host:port/db
GEMINI_API_KEY=your_api_key
REDIS_URL=redis://localhost:6379
ENABLE_AUTO_REPORT_SCHEDULER=false
DEBUG_MODE=false
```

### Configuration Loading
```python
from dotenv import load_dotenv
import os

# Load environment
load_dotenv()

# Configuration access
DATABASE_URL = os.getenv("DATABASE_URL", "sqlite:///./ai_report.db")
GEMINI_API_KEY = os.getenv("GEMINI_API_KEY")
```

### Feature Flags
```python
# Conditional features
ENABLE_AUTO_SCHEDULER = os.getenv("ENABLE_AUTO_REPORT_SCHEDULER", "false").lower() == "true"
DEBUG_MODE = os.getenv("DEBUG_MODE", "false").lower() == "true"
```

## 🚀 Deployment Architecture

### Container Setup
```dockerfile
# Multi-stage build
FROM python:3.11-slim as builder
# Dependencies installation

FROM python:3.11-slim as runner  
# Runtime environment
COPY --from=builder /app /app
EXPOSE 8000
CMD ["python", "run.py"]
```

### Service Orchestration
```yaml
# docker-compose.yml
services:
  app:
    build: .
    ports:
      - "8000:8000"
    environment:
      - DATABASE_URL=postgresql://postgres:password@db:5432/reports
    depends_on:
      - db
      
  db:
    image: postgres:15
    environment:
      POSTGRES_DB: reports
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
```

### Health Monitoring
```python
@app.get("/health")
async def health_check():
    """System health endpoint"""
    try:
        # Database check
        with engine.connect() as conn:
            conn.execute(text("SELECT 1"))
        
        return {
            "status": "healthy",
            "database": "connected",
            "timestamp": datetime.utcnow()
        }
    except Exception as e:
        return {
            "status": "unhealthy", 
            "error": str(e)
        }
```

## 🔒 Security Architecture

### API Security
```python
# Input validation với Pydantic
class ReportRequest(BaseModel):
    session_id: Optional[str] = None
    
    class Config:
        str_strip_whitespace = True
        max_anystr_length = 1000
```

### Database Security
- **Connection encryption:** SSL/TLS cho PostgreSQL
- **SQL injection protection:** Parameterized queries only
- **Access control:** Environment-based credentials
- **Connection pooling:** Controlled resource access

### Environment Security
```python
# Sensitive data từ environment
GEMINI_API_KEY = os.getenv("GEMINI_API_KEY")
if not GEMINI_API_KEY:
    raise ValueError("GEMINI_API_KEY is required")
```

## 📊 Performance Architecture

### Async Processing
```python
# FastAPI async endpoints
@router.post("/auto-generate")
async def generate_report(background_tasks: BackgroundTasks):
    # Non-blocking response
    background_tasks.add_task(run_workflow, session_id)
    return {"session_id": session_id}
```

### Concurrent Operations
```python
# Parallel API calls
with ThreadPoolExecutor(max_workers=3) as executor:
    futures = [
        executor.submit(get_global_data),
        executor.submit(get_btc_data), 
        executor.submit(get_fng_data)
    ]
    results = [f.result(timeout=10) for f in futures]
```

### Resource Management
- **Database pooling:** 10 base connections, 20 overflow
- **Memory management:** Session cleanup after requests
- **Timeout protection:** 10s API timeouts, 60s AI generation

### Caching Strategy
```python
# In-memory caching cho real-time data
@lru_cache(maxsize=100)
def get_cached_market_data():
    return fetch_market_data()

# TTL-based cache invalidation
cache_expire = datetime.utcnow() + timedelta(minutes=5)
```

## 🔍 Monitoring & Observability

### Logging Architecture
```python
import logging

# Structured logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)

logger = logging.getLogger(__name__)
logger.info(f"Processing request {session_id}")
```

### Metrics Collection
```python
# Progress tracking metrics
progress_tracker.update_step(
    session_id=session_id,
    step=current_step,
    total_steps=total_steps,
    details="Processing...",
    timestamp=datetime.utcnow()
)
```

### Error Tracking
```python
# Centralized error handling
try:
    result = process_workflow(state)
except Exception as e:
    logger.error(f"Workflow failed: {e}", exc_info=True)
    progress_tracker.error_progress(session_id, str(e))
    raise
```

## 🔄 Scalability Considerations

### Horizontal Scaling
- **Stateless design:** Session state trong database
- **Load balancing ready:** Multiple FastAPI instances
- **Database scaling:** Connection pooling + read replicas

### Vertical Scaling
- **Memory optimization:** Efficient data structures
- **CPU optimization:** Concurrent processing
- **I/O optimization:** Async operations

### Future Enhancements
- **Message queues:** Redis/RabbitMQ cho background tasks
- **Caching layer:** Redis cho session state
- **API rate limiting:** Request throttling
- **Container orchestration:** Kubernetes deployment

---
**Version:** 2.0  
**Architecture Pattern:** Layered + Microservice-like  
**Last Updated:** 18/08/2025
