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

# Copy application code from builder
COPY --from=builder /app .

# Install Python dependencies in runtime stage to ensure entrypoints are available
COPY requirements.txt ./
RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir -r requirements.txt

# Create data directory for database
RUN mkdir -p /app/data

# Create non-root user for security
RUN useradd -r -u 1001 -m appuser && chown -R appuser:appuser /app
USER appuser

# Expose port
EXPOSE 8888

# Health check for API endpoint
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD curl -f http://localhost:8888/health || exit 1

# Environment variables with defaults
ENV PYTHONUNBUFFERED=1 \
    PYTHONDONTWRITEBYTECODE=1 \
    DATABASE_URL="sqlite:///./data/ai_report.db" \
    HOST="0.0.0.0" \
    PORT="8888"

# Entrypoint - show quick info and start uvicorn
COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
RUN chmod +x /usr/local/bin/docker-entrypoint.sh
ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8888"]
