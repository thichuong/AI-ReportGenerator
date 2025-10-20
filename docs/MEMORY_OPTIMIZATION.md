# Memory Optimization - Tối ưu hóa Bộ nhớ

## Tổng quan

Tài liệu này mô tả các cải tiến về quản lý bộ nhớ đã được thực hiện cho AI Report Generator để giảm memory leak và tối ưu hóa hiệu suất hệ thống.

## Vấn đề

Trước đây, ứng dụng gặp vấn đề tích lũy bộ nhớ (memory leak) do:
1. Kết nối Redis không được đóng đúng cách
2. Dữ liệu lớn (HTML, CSS, JS content) được giữ trong bộ nhớ sau khi lưu vào database
3. Không có cơ chế garbage collection chủ động

## Giải pháp đã triển khai

### 1. Market Indices Scheduler (`app/routers/market.py`)

**Cải tiến:**
- ✅ Sử dụng `async with` context manager cho Redis connections
- ✅ Tự động đóng kết nối sau khi sử dụng
- ✅ Xóa dữ liệu tạm (`data` dictionary) sau khi lưu vào Redis
- ✅ Thêm `gc.collect()` trong API endpoint

**Code:**
```python
async with redis.Redis.from_url(redis_url) as redis_client:
    await redis_client.set('market_indices', json.dumps(response_data), ex=300)

# Giải phóng bộ nhớ
del data
gc.collect()
```

**Lợi ích:**
- Giảm memory footprint mỗi lần chạy (mỗi 3 phút)
- Ngăn chặn tích lũy kết nối Redis
- Tự động thu hồi bộ nhớ không sử dụng

### 2. Database Save Node (`app/services/workflow_nodes/save_database.py`)

**Cải tiến:**
- ✅ Import `gc` module
- ✅ Xóa các trường dữ liệu lớn từ state sau khi lưu thành công
- ✅ Gọi `gc.collect()` sau mọi thao tác (thành công hoặc thất bại)
- ✅ Xóa object `new_report` sau khi commit

**Code:**
```python
# Sau khi lưu thành công
large_fields = ["html_content", "css_content", "js_content", 
              "html_content_en", "js_content_en"]
for field in large_fields:
    if field in state:
        del state[field]

del new_report
gc.collect()
```

**Lợi ích:**
- Giải phóng ngay lập tức các nội dung HTML/CSS/JS lớn (có thể hàng MB)
- Giảm memory footprint của state object
- Cải thiện hiệu suất cho các báo cáo tiếp theo

### 3. Auto Report Scheduler (`app/services/auto_report_scheduler.py`)

**Cải tiến:**
- ✅ Import `gc` module
- ✅ Giải phóng bộ nhớ sau mỗi lần tạo báo cáo (thành công hoặc thất bại)
- ✅ Thêm cleanup trong exception handlers
- ✅ Log rõ ràng về memory cleanup

**Code:**
```python
# Sau khi tạo báo cáo thành công
del result
gc.collect()
print(f"🧹 Memory cleanup completed")

# Sau khi retry hoặc error
gc.collect()
```

**Lợi ích:**
- Scheduler chạy liên tục không bị memory leak
- Giảm nguy cơ crash do hết bộ nhớ
- Tăng độ ổn định cho long-running processes

## Kết quả mong đợi

### Trước khi tối ưu:
- Memory tăng dần theo thời gian
- Kết nối Redis tích lũy
- Có thể crash sau vài giờ/ngày chạy liên tục

### Sau khi tối ưu:
- Memory ổn định, không tăng theo thời gian
- Kết nối Redis được quản lý đúng cách
- Có thể chạy ổn định 24/7

## Monitoring

Để theo dõi hiệu quả của các cải tiến:

1. **Kiểm tra memory usage:**
```bash
# Trong container Docker
docker stats <container_id>

# Hoặc xem logs
docker logs -f <container_id> | grep "Memory cleanup"
```

2. **Kiểm tra Redis connections:**
```bash
redis-cli INFO clients
```

3. **Monitor trong Python:**
```python
import psutil
import os

process = psutil.Process(os.getpid())
print(f"Memory usage: {process.memory_info().rss / 1024 / 1024:.2f} MB")
```

## Best Practices

1. **Luôn sử dụng context managers** cho connections (Redis, Database)
2. **Xóa dữ liệu lớn** ngay sau khi không cần dùng
3. **Gọi gc.collect()** sau các operations lớn
4. **Log memory cleanup** để dễ debug
5. **Monitor memory usage** trong production

## Các điểm cần lưu ý

⚠️ **Không nên:**
- Gọi `gc.collect()` quá thường xuyên (có thể ảnh hưởng performance)
- Xóa dữ liệu khi còn cần dùng
- Quên đóng connections

✅ **Nên:**
- Sử dụng `async with` cho async resources
- Xóa dữ liệu ngay sau khi lưu xong
- Có error handling cho cleanup logic

## Changelog

### 2025-10-20
- ✅ Thêm memory cleanup cho market indices scheduler
- ✅ Thêm memory cleanup cho save database node
- ✅ Thêm memory cleanup cho auto report scheduler
- ✅ Cải thiện Redis connection management
- ✅ Thêm logging cho memory cleanup operations

## Tham khảo

- [Python Garbage Collection](https://docs.python.org/3/library/gc.html)
- [Redis-py Best Practices](https://redis-py.readthedocs.io/en/stable/)
- [SQLAlchemy Session Management](https://docs.sqlalchemy.org/en/14/orm/session_basics.html)
