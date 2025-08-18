# Multi-stage build optimized Dockerfile for AI-ReportGenerator
FROM python:3.11-slim-bookworm as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    build-essential \
    gcc \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Update CA certificates
RUN update-ca-certificates

WORKDIR /app

# Copy requirements first for better caching
COPY requirements.txt ./

# Install Python dependencies in builder stage
RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir -r requirements.txt

# Copy source code
COPY . .

# Runtime stage - minimal slim image
FROM python:3.11-slim-bookworm

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Python dependencies from builder
COPY --from=builder /usr/local/lib/python3.11/site-packages /usr/local/lib/python3.11/site-packages
COPY --from=builder /usr/local/bin /usr/local/bin

# Copy application code from builder
COPY --from=builder /app .

# Create data directory for database
RUN mkdir -p /app/data

# Create non-root user for security
RUN useradd -r -u 1001 -m appuser && chown -R appuser:appuser /app
USER appuser

# Expose port
EXPOSE 8888

# Health check for API endpoint - wait longer for startup
HEALTHCHECK --interval=30s --timeout=30s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8888/health || exit 1

# Environment variables with defaults
ENV PYTHONUNBUFFERED=1 \
    PYTHONDONTWRITEBYTECODE=1 \
    DATABASE_URL="sqlite:///./data/ai_report.db" \
    HOST="0.0.0.0" \
    PORT="8888"

# Command to run the application
CMD ["python", "run.py"]
