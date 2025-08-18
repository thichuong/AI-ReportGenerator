#!/bin/bash

# Exit on any error
set -e

# Function to log messages
log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1"
}

log "🚀 Starting AI Report Generator Docker Container..."

# Check if DATABASE_URL is set, if not use SQLite
if [ -z "$DATABASE_URL" ]; then
    log "⚠️  DATABASE_URL not set, using SQLite database..."
    export DATABASE_URL="sqlite:///app/ai_report.db"
    
    # Create data directory if it doesn't exist
    mkdir -p /app/data
else
    log "✅ Using PostgreSQL database from DATABASE_URL"
fi

# Set default port if not provided
if [ -z "$PORT" ]; then
    export PORT=8888
    log "⚠️  PORT not set, using default port: $PORT"
else
    log "✅ Using port: $PORT"
fi

# Set default host
if [ -z "$HOST" ]; then
    export HOST="0.0.0.0"
fi

log "🌍 Environment variables:"
log "   HOST: $HOST"
log "   PORT: $PORT"
log "   DATABASE_URL: ${DATABASE_URL:0:20}..." # Only show first 20 chars for security

# Execute the command
log "🚀 Starting application..."
exec "$@"
