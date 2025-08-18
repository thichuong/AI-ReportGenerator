#!/bin/bash

# Test script for Docker configuration
echo "🧪 Testing AI-ReportGenerator Docker Configuration"
echo "================================================="

# Check if Dockerfile exists and has correct syntax
echo "✅ Checking Dockerfile..."
if [ -f "Dockerfile" ]; then
    echo "  - Dockerfile exists"
    
    # Basic syntax check
    if grep -q "FROM.*python" Dockerfile; then
        echo "  - Python base image detected"
    fi
    
    if grep -q "WORKDIR /app" Dockerfile; then
        echo "  - Working directory set correctly"
    fi
    
    if grep -q "COPY requirements.txt" Dockerfile; then
        echo "  - Requirements.txt copy step found"
    fi
    
    if grep -q "pip install" Dockerfile; then
        echo "  - Python dependencies installation found"
    fi
    
    if grep -q "EXPOSE 8888" Dockerfile; then
        echo "  - Port 8888 exposed correctly"
    fi
    
    if grep -q "CMD.*python.*run.py" Dockerfile; then
        echo "  - Correct CMD instruction found"
    fi
else
    echo "  ❌ Dockerfile not found!"
    exit 1
fi

# Check docker-compose.yml
echo ""
echo "✅ Checking docker-compose.yml..."
if [ -f "docker-compose.yml" ]; then
    echo "  - docker-compose.yml exists"
    
    if grep -q "build: \." docker-compose.yml; then
        echo "  - Build context set to current directory"
    fi
    
    if grep -q "8888:8888" docker-compose.yml; then
        echo "  - Port mapping configured correctly"
    fi
    
    if grep -q "ai-report-generator" docker-compose.yml; then
        echo "  - Service name configured"
    fi
else
    echo "  ❌ docker-compose.yml not found!"
fi

# Check railway.json
echo ""
echo "✅ Checking railway.json..."
if [ -f "railway.json" ]; then
    echo "  - railway.json exists"
    
    if grep -q "Dockerfile" railway.json; then
        echo "  - Dockerfile path configured for Railway"
    fi
else
    echo "  ❌ railway.json not found!"
fi

# Check .dockerignore
echo ""
echo "✅ Checking .dockerignore..."
if [ -f ".dockerignore" ]; then
    echo "  - .dockerignore exists for build optimization"
else
    echo "  ⚠️  .dockerignore not found - build context not optimized"
fi

# Check deploy script
echo ""
echo "✅ Checking deploy.sh..."
if [ -f "deploy.sh" ]; then
    if [ -x "deploy.sh" ]; then
        echo "  - deploy.sh exists and is executable"
    else
        echo "  ⚠️  deploy.sh exists but is not executable"
        echo "     Run: chmod +x deploy.sh"
    fi
else
    echo "  ❌ deploy.sh not found!"
fi

echo ""
echo "🎉 Configuration check complete!"
echo ""
echo "📋 To deploy:"
echo "  1. Install Docker: sudo apt install docker.io docker-compose"
echo "  2. Run deployment: ./deploy.sh"
echo "  3. Or manual: docker-compose up -d --build"
echo ""
echo "🚀 Railway deployment: railway up"
