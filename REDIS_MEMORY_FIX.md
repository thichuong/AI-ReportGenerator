# Redis Memory Leak Fix - base.py

## 🔴 Vấn đề phát hiện

**File:** `app/services/workflow_nodes/base.py`  
**Function:** `get_realtime_dashboard_data()` (dòng 250-381)  
**Severity:** 🔥 **CRITICAL** - Memory Leak

### Mô tả vấn đề:

Kết nối Redis được tạo nhưng **KHÔNG BAO GIỜ được đóng**:

```python
# BEFORE - Code cũ có BUG:
r = redis.Redis.from_url(redis_url)
cached_data = r.get('latest_market_data')
# ❌ Không có r.close() hoặc context manager!
# ❌ Connection leak!
```

### Hậu quả:

- ✗ **Memory leak** khi hàm được gọi nhiều lần
- ✗ **Kết nối Redis tích lũy** không được giải phóng  
- ✗ Có thể gây **crash** sau thời gian chạy dài
- ✗ Redis server có thể từ chối kết nối mới do quá nhiều connections
- ✗ **Không nhất quán** với các memory optimizations khác trong project

---

## ✅ Giải pháp đã áp dụng

### 1. Thêm finally block để đóng connection

```python
# AFTER - Code mới đã FIX:
def get_realtime_dashboard_data():
    """Lấy dữ liệu crypto thời gian thực từ Redis với key 'latest_market_data' và tự động giải phóng bộ nhớ"""
    max_retries = 3
    retry_delay = 2
    
    for attempt in range(max_retries):
        r = None  # Initialize để đảm bảo cleanup trong finally
        try:
            redis_url = os.getenv('REDIS_URL')
            if not redis_url:
                print("Warning: REDIS_URL not set, cannot fetch market data")
                return None
                
            r = redis.Redis.from_url(redis_url)
            cached_data = r.get('latest_market_data')
            
            # ... xử lý data ...
            
        except Exception as e:
            print(f"Error getting cached crypto data from Redis: {e}")
            return None
        finally:
            # ✅ Đóng kết nối Redis để tránh memory leak
            if r is not None:
                try:
                    r.close()
                    print("🧹 Redis connection closed")
                except Exception as e:
                    print(f"Error closing Redis connection: {e}")
```

### 2. Cập nhật docstring

Thêm mô tả "tự động giải phóng bộ nhớ" vào docstring để rõ ràng hơn.

---

## 🎯 Lợi ích

| Trước | Sau |
|-------|-----|
| ❌ Connection leak mỗi lần gọi | ✅ Connection được đóng đúng cách |
| ❌ Memory tăng dần theo thời gian | ✅ Memory ổn định |
| ❌ Có thể crash sau nhiều lần gọi | ✅ Chạy ổn định 24/7 |
| ❌ Redis server có thể quá tải | ✅ Số connections được kiểm soát |
| ❌ Không nhất quán với code khác | ✅ Nhất quán với memory optimization |

---

## 🧪 Cách kiểm tra

### 1. Chạy test script:
```bash
cd /home/thichuong/Desktop/AI-ReportGenerator
python test_redis_memory_fix.py
```

### 2. Kiểm tra Redis connections:
```bash
# Trước khi chạy app
redis-cli INFO clients | grep connected_clients

# Sau khi chạy app một thời gian
redis-cli INFO clients | grep connected_clients

# Số connections nên ổn định, không tăng dần
```

### 3. Monitor memory trong production:
```bash
# Docker stats
docker stats <container_id>

# Logs - nên thấy "🧹 Redis connection closed"
docker logs -f <container_id> | grep "Redis connection"
```

### 4. Manual test trong Python:
```python
import psutil
import os
from app.services.workflow_nodes.base import get_realtime_dashboard_data

process = psutil.Process(os.getpid())
print(f"Memory before: {process.memory_info().rss / 1024 / 1024:.2f} MB")

for i in range(10):
    data = get_realtime_dashboard_data()
    print(f"Call {i+1}: OK")

print(f"Memory after: {process.memory_info().rss / 1024 / 1024:.2f} MB")
# Memory không nên tăng đáng kể
```

---

## 📊 So sánh với các file khác trong project

### File `app/routers/market.py` - ✅ ĐÃ ĐÚNG:
```python
# Sử dụng async context manager
async with redis.Redis.from_url(redis_url) as redis_client:
    await redis_client.set('market_indices', json.dumps(response_data), ex=300)
# ✅ Connection tự động đóng khi ra khỏi context
```

### File `base.py` - ❌ TRƯỚC ĐÂY SAI → ✅ ĐÃ SỬA:
```python
# TRƯỚC: Synchronous code, không dùng context manager
r = redis.Redis.from_url(redis_url)
cached_data = r.get('latest_market_data')
# ❌ Không có cleanup

# SAU: Thêm finally block
r = None
try:
    r = redis.Redis.from_url(redis_url)
    cached_data = r.get('latest_market_data')
    # ... process ...
finally:
    if r is not None:
        r.close()
# ✅ Connection được đóng
```

**Lý do không dùng context manager trong base.py:**
- Code là **synchronous** (không có async/await)
- `redis.Redis` không hỗ trợ context manager cho sync mode
- Phải dùng `try-finally` với `r.close()` thủ công

---

## 🔄 Migration / Deployment

### Không cần migration:
- ✅ Thay đổi chỉ ở internal logic
- ✅ Không thay đổi API
- ✅ Không thay đổi database schema
- ✅ Backward compatible 100%

### Deploy ngay:
```bash
# Git commit
git add app/services/workflow_nodes/base.py
git add test_redis_memory_fix.py
git add REDIS_MEMORY_FIX.md
git commit -m "fix: Close Redis connection in get_realtime_dashboard_data to prevent memory leak"

# Deploy
git push

# Railway/Docker sẽ tự động restart
```

### Verify sau deploy:
```bash
# 1. Check logs
docker logs -f <container_id> | grep "🧹 Redis connection closed"

# 2. Monitor memory
watch -n 5 'docker stats --no-stream <container_id>'

# 3. Check Redis connections
redis-cli INFO clients
```

---

## 📝 Related Issues

### Memory Optimization đã có trong project:
1. ✅ `app/routers/market.py` - Redis async context manager
2. ✅ `app/services/workflow_nodes/save_database.py` - State cleanup
3. ✅ `app/services/auto_report_scheduler.py` - Result cleanup
4. ✅ **NEW:** `app/services/workflow_nodes/base.py` - Redis connection cleanup

### Tài liệu liên quan:
- `docs/MEMORY_OPTIMIZATION.md`
- `MEMORY_OPTIMIZATION_SUMMARY.md`
- `CHANGELOG_MEMORY_OPTIMIZATION.md`
- `test_memory_optimization.py`

---

## ⚠️ Rollback Plan

Nếu có vấn đề sau khi deploy:

### Option 1: Git revert
```bash
git revert <commit-hash>
git push
```

### Option 2: Manual fix
```python
# Xóa finally block nếu gây lỗi
# Nhưng điều này KHÔNG NÊN vì sẽ quay lại memory leak
```

### Monitoring sau rollback:
- Theo dõi Redis connections tăng dần
- Theo dõi memory usage tăng dần
- Chuẩn bị fix lại với approach khác nếu cần

---

## 🎓 Best Practices Applied

1. ✅ **Always close connections** - Luôn đóng kết nối sau khi dùng
2. ✅ **Use try-finally** - Đảm bảo cleanup ngay cả khi có exception
3. ✅ **Initialize to None** - Để kiểm tra an toàn trong finally
4. ✅ **Log cleanup operations** - Để dễ debug và monitor
5. ✅ **Handle close errors** - Không để lỗi cleanup làm crash app
6. ✅ **Consistent with project** - Giống với các memory optimizations khác

---

## 🚀 Impact

### Immediate:
- ✅ Không còn connection leak
- ✅ Memory usage ổn định
- ✅ Redis server không quá tải

### Long-term:
- ✅ App có thể chạy 24/7 không crash
- ✅ Giảm chi phí infrastructure (ít memory hơn)
- ✅ Tăng reliability và uptime

### Metrics to monitor:
- **Redis connections**: Nên ổn định ở mức thấp
- **Memory RSS**: Không tăng theo thời gian
- **App uptime**: Không crash do OOM
- **Response time**: Không chậm dần do memory pressure

---

**Fixed by:** Memory Optimization Team  
**Date:** 2025-10-22  
**Status:** ✅ Fixed and Tested  
**Priority:** 🔥 Critical
