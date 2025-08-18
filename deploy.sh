#!/bin/bash

# Deploy script for AI-ReportGenerator
# Based on web-server-report deployment pattern

set -e

echo "🚀 AI-ReportGenerator Deployment Script"
echo "========================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    print_error "Docker is not running. Please start Docker first."
    exit 1
fi

# Check if docker-compose is available
if ! command -v docker-compose &> /dev/null; then
    if ! docker compose version &> /dev/null; then
        print_error "Docker Compose is not available. Please install docker-compose or use Docker with compose plugin."
        exit 1
    else
        DOCKER_COMPOSE_CMD="docker compose"
    fi
else
    DOCKER_COMPOSE_CMD="docker-compose"
fi

print_status "Using Docker Compose command: $DOCKER_COMPOSE_CMD"

# Create necessary directories
print_status "Creating necessary directories..."
mkdir -p data logs

# Set proper permissions
chmod 755 data logs

print_success "Directories created successfully"

# Build and start services
print_status "Building and starting AI-ReportGenerator services..."
$DOCKER_COMPOSE_CMD down --remove-orphans
$DOCKER_COMPOSE_CMD build --no-cache
$DOCKER_COMPOSE_CMD up -d

print_success "Services started successfully"

# Wait for health check
print_status "Waiting for health check..."
sleep 15

# Check if service is healthy with more detail
print_status "Checking service health..."
for i in {1..6}; do
    if curl -f http://localhost:8888/health > /dev/null 2>&1; then
        print_success "Health check passed - AI-ReportGenerator is running!"
        break
    else
        print_warning "Health check attempt $i/6 failed. Waiting 10s..."
        if [ $i -eq 6 ]; then
            print_error "Health check failed after 6 attempts"
            print_status "Container logs:"
            $DOCKER_COMPOSE_CMD logs --tail=50 ai-report-generator
            print_status "You can run ./health-debug.sh for more detailed diagnostics"
        else
            sleep 10
        fi
    fi
done

echo ""
echo "🎉 Deployment complete!"
echo "========================================"
echo "📊 AI-ReportGenerator Dashboard: http://localhost:8888"
echo "🔍 Health Check: http://localhost:8888/health"
echo "📝 View Logs: $DOCKER_COMPOSE_CMD logs -f"
echo "🛑 Stop Services: $DOCKER_COMPOSE_CMD down"
echo ""
echo "📁 Data Directory: ./data"
echo "📋 Logs Directory: ./logs"
echo ""
print_success "AI-ReportGenerator is ready to use!"
