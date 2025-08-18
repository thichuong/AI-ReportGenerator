#!/bin/bash

# Simple health check script for debugging
echo "🏥 Health Check Debug Script"
echo "============================"

# Test various endpoints
echo "1. Testing localhost:8888/health..."
curl -v http://localhost:8888/health 2>&1 || echo "❌ Failed to connect to localhost:8888/health"

echo ""
echo "2. Testing 127.0.0.1:8888/health..."
curl -v http://127.0.0.1:8888/health 2>&1 || echo "❌ Failed to connect to 127.0.0.1:8888/health"

echo ""
echo "3. Testing 0.0.0.0:8888/health..."
curl -v http://0.0.0.0:8888/health 2>&1 || echo "❌ Failed to connect to 0.0.0.0:8888/health"

echo ""
echo "4. Check if port 8888 is listening..."
netstat -tlnp | grep :8888 || echo "❌ Port 8888 not found in netstat"

echo ""
echo "5. Check running processes..."
ps aux | grep python | grep -v grep

echo ""
echo "6. Test root endpoint..."
curl -v http://localhost:8888/ 2>&1 || echo "❌ Failed to connect to root endpoint"
