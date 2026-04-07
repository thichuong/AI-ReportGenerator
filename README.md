# AI Report Generator (Rust Edition)

## 📝 Description
AI Report Generator is a high-performance application built with Rust and Axum for automating investment report generation. It integrates with Google Gemini AI to analyze market data and create comprehensive reports.

## 🚀 Features
- **Fast and Type-Safe**: Implemented in Rust using the Axum web framework.
- **AI-Powered Analysis**: Leverages Google Gemini AI for intelligent content generation.
- **Real-time Data Integration**: Fetches live market data from Redis Streams.
- **Workflow Orchestration**: Automated multi-step report generation workflow.
- **Built-in Scheduler**: Automated report generation at configurable intervals.
- **Progress Tracking**: Real-time status updates for ongoing generation tasks.
- **PostgreSQL Integration**: Persistent storage for generated reports.
- **Modern Dashboard**: Built-in web interface for viewing and managing reports.

## 📁 Project Structure
```text
AI-ReportGenerator/
├── src/
│   ├── api/          # Axum handlers and request routing
│   ├── db/           # PostgreSQL models and database logic
│   ├── scheduler/    # Automated task scheduling
│   ├── workflow/     # AI report generation logic and nodes
│   ├── static/       # Frontend assets (HTML, CSS, JS)
│   ├── main.rs       # Entry point
│   └── lib.rs        # Core logic exports
├── prompt_envs/      # AI prompts and template configurations
├── Cargo.toml        # Rust dependencies and project metadata
└── .env.example      # Environment configuration template
```

## 🛠️ Getting Started

### 1. Prerequisites
- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install) (stable version recommended)
- **PostgreSQL**: Required for data storage.
- **Redis**: Required for real-time market data integration.

### 2. Installation
```bash
# Clone the repository
git clone <repository-url>
cd AI-ReportGenerator

# Copy environment template
cp .env.example .env
```

### 3. Configuration
Edit the `.env` file and provide your credentials:
- `DATABASE_URL`: PostgreSQL connection string.
- `GEMINI_API_KEY`: Your Google AI API key.
- `REDIS_URL`: Redis server address.

### 4. Running the Application
```bash
# Run in development mode
cargo run

# Build for production
cargo build --release
./target/release/ai-report-generator
```

## 🌐 API Endpoints

### 📊 Report Generation
- `POST /api/v1/generate-auto-report` - Start background report generation.
- `GET /api/v1/progress/{session_id}` - Track the progress of a generation session.
- `POST /api/v1/manual-generate` - Trigger an immediate, synchronous report generation.

### 📈 Monitoring & Status
- `GET /api/v1/scheduler/status` - Check the status of the auto-report scheduler.
- `GET /api/v1/reports/latest` - Retrieve metadata for the most recently generated report.
- `GET /health` - Application health check.

### 🖥️ Dashboard
- `GET /` - Redirects to the built-in web dashboard.
- `GET /static/*` - Serves frontend assets.

## 🚢 Deployment (CI/CD)
The project includes a GitHub Actions workflow for automatic deployment to a Google Cloud VM.

### Configuration
Set the following secrets in your GitHub repository:
- `SSH_HOST`: VM Public IP.
- `SSH_USER`: SSH username.
- `SSH_KEY`: SSH private key.
- `GCP_SA_KEY`: Google Cloud Service Account key (JSON).
- `GCP_PROJECT`: GCP Project ID.
- `GCP_VM_NAME`: Target VM Name.
- `GCP_ZONE`: Target VM Zone.

The workflow will automatically build the release binary, transfer it via IAP, and restart the service on every push to the `main` branch.

## 📄 License
This project is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.