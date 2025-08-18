# AI-ReportGenerator Docker Deployment

## 🐳 Docker Files Structure

- `Dockerfile` - Multi-stage optimized build (main deployment)
- `Dockerfile.optimized` - Ubuntu-based alternative build
- `docker-compose.yml` - Docker Compose configuration
- `deploy.sh` - Automated deployment script
- `railway.json` - Railway deployment configuration
- `.dockerignore` - Docker build context optimization

## 🚀 Quick Deployment

### 1. Local Docker Deployment (Recommended)
```bash
# Run the deployment script
./deploy.sh
```

This script will:
- ✅ Check Docker availability
- ✅ Create necessary directories
- ✅ Build optimized Docker image
- ✅ Start services with health checks
- ✅ Verify deployment

### 2. Manual Docker Compose
```bash
# Create directories
mkdir -p data logs

# Build and start
docker-compose up -d --build

# Check logs
docker-compose logs -f

# Stop services
docker-compose down
```

### 3. Railway Deployment
```bash
# Install Railway CLI
npm install -g @railway/cli

# Login and deploy
railway login
railway link
railway up
```

## 🔧 Configuration

### Environment Variables
```env
DATABASE_URL=sqlite:///./data/ai_report.db
HOST=0.0.0.0
PORT=8888
DEBUG=False
```

### Docker Compose Services
- **ai-report-generator**: Main application service
- **Port**: 8888
- **Health Check**: `/health` endpoint
- **Volumes**: 
  - `./data:/app/data` - Database storage
  - `./logs:/app/logs` - Application logs

## 📊 Monitoring

### Health Check
```bash
curl http://localhost:8888/health
```

### Service Status
```bash
docker-compose ps
docker-compose logs -f ai-report-generator
```

### Performance Monitoring
```bash
# Container stats
docker stats

# Resource usage
docker-compose top
```

## 🎯 Optimizations (vs Basic Docker)

✅ **Multi-stage build**: Reduced image size (~60% smaller)  
✅ **Layer caching**: Faster rebuilds with dependency caching  
✅ **Security**: Non-root user, minimal attack surface  
✅ **Health checks**: Built-in container health monitoring  
✅ **Distroless runtime**: Minimal base image  
✅ **CA certificates**: Proper SSL/TLS support  
✅ **Build context**: Optimized with .dockerignore  

## 🔍 Troubleshooting

### Container won't start
```bash
docker-compose logs ai-report-generator
```

### Health check failing
```bash
# Check if port is accessible
curl -v http://localhost:8888/health

# Check container status
docker-compose ps
```

### Permission issues
```bash
# Fix data directory permissions
sudo chown -R $USER:$USER data logs
```

### Build issues
```bash
# Clean build
docker-compose down
docker system prune -f
docker-compose build --no-cache
```

## 📁 Directory Structure

```
AI-ReportGenerator/
├── 🐳 Dockerfile                 # Basic Docker config
├── 🐳 Dockerfile.optimized       # Multi-stage optimized (recommended)
├── 🐳 docker-compose.yml         # Compose configuration
├── 🚀 deploy.sh                  # Automated deployment
├── 🚂 railway.json              # Railway deployment
├── 📝 .dockerignore              # Build context optimization
├── 📊 data/                      # Database storage (mounted)
└── 📋 logs/                      # Application logs (mounted)
```

## 🎉 Success Indicators

After successful deployment:
- 🟢 Health check at http://localhost:8888/health returns OK
- 🟢 API accessible at http://localhost:8888
- 🟢 Database created in ./data directory
- 🟢 Logs available in ./logs directory
- 🟢 Container shows healthy status in `docker ps`
