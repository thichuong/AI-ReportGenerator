# Auto Report Scheduler - Fixed Times

## Tổng quan

Scheduler đã được cập nhật để chạy báo cáo theo khung giờ cố định thay vì interval. 

## Khung giờ mặc định (Giờ Việt Nam - GMT+7):
- **07:30** - Báo cáo buổi sáng
- **15:00** - Báo cáo buổi chiều  
- **19:00** - Báo cáo buổi tối

## Cấu hình Environment Variables

### Bật scheduler:
```bash
ENABLE_AUTO_REPORT_SCHEDULER=true
```

### Custom khung giờ (tùy chọn):
```bash
AUTO_REPORT_SCHEDULE_TIMES="07:30,15:00,19:00"
```

### Số lần thử tối đa:
```bash
MAX_REPORT_ATTEMPTS=3
```

### API Key:
```bash
GEMINI_API_KEY=your_api_key_here
```

## Cách hoạt động

1. **Tính toán thời gian tiếp theo**: Scheduler sẽ tự động tính toán khung giờ tiếp theo dựa trên thời gian hiện tại
2. **Sleep đến đúng giờ**: Chờ đến đúng khung giờ đã định
3. **Tolerance check**: Kiểm tra xem có đúng khung giờ không (tolerance ±5 phút)
4. **Chạy báo cáo**: Nếu đúng khung giờ, sẽ tạo báo cáo tự động
5. **Error handling**: Nếu thất bại liên tiếp 3 lần, skip tới khung giờ tiếp theo

## Tính năng mới

### 1. Timezone Việt Nam
- Sử dụng `pytz` để đảm bảo chính xác timezone GMT+7
- Tất cả log và thời gian đều hiển thị theo giờ Việt Nam

### 2. Flexible Schedule
- Có thể cấu hình khung giờ custom qua environment variable
- Format: `"HH:MM,HH:MM,HH:MM"`

### 3. Smart Time Calculation
- Tự động tìm khung giờ tiếp theo trong ngày
- Nếu hết khung giờ trong ngày, chuyển sang ngày mai

### 4. Tolerance Check
- Kiểm tra với tolerance ±5 phút để tránh drift
- Đảm bảo báo cáo chỉ chạy đúng khung giờ

### 5. Improved Error Handling
- Track consecutive failures
- Skip tới khung giờ tiếp theo nếu thất bại quá nhiều
- Memory cleanup sau mỗi lần chạy

## Test Results

Tất cả test cases đã pass:
- ✅ Time calculation logic
- ✅ Tolerance check (±5 minutes)
- ✅ Next day rollover
- ✅ Edge cases

## Logs mẫu

```
[2025-10-22 12:56:45.123456+07:00] 🎯 Auto report scheduler khởi động với khung giờ: 07:30, 15:00, 19:00 (GMT+7)
[2025-10-22 12:56:45.123456+07:00] ⏰ Scheduler: Chạy tiếp theo vào 2025-10-22 15:00:00 +07
[2025-10-22 15:00:00.123456+07:00] 🚀 Scheduler: Bắt đầu tạo báo cáo tự động (khung 15:00)...
[2025-10-22 15:05:30.123456+07:00] ✅ Scheduler: Báo cáo #123 tạo thành công trong 330.2s (khung 15:00)
[2025-10-22 15:05:30.123456+07:00] 🧹 Memory cleanup completed
[2025-10-22 15:05:30.123456+07:00] ⏰ Scheduler: Chạy tiếp theo vào 2025-10-22 19:00:00 +07
```

## Migration từ interval-based

Nếu bạn đang sử dụng scheduler cũ với `AUTO_REPORT_INTERVAL_HOURS`, chỉ cần:

1. Xóa `AUTO_REPORT_INTERVAL_HOURS` (không còn dùng)
2. Thiết lập `AUTO_REPORT_SCHEDULE_TIMES` nếu muốn custom (optional)
3. Restart application

## Dependencies

Đã thêm vào `requirements.txt`:
```
pytz>=2023.3
```