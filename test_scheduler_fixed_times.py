#!/usr/bin/env python3
"""
Test script để kiểm tra logic scheduler mới với khung giờ cố định.
"""

import os
import sys
from datetime import datetime, timedelta
import pytz

# Thêm path để import module
sys.path.append('/home/thichuong/Desktop/AI-ReportGenerator')

def test_get_next_run_time():
    """Test hàm tính toán thời gian chạy tiếp theo."""
    
    # Timezone Việt Nam
    vietnam_tz = pytz.timezone('Asia/Ho_Chi_Minh')
    schedule_times = ["07:30", "15:00", "19:00"]
    
    def get_next_run_time(current_time=None):
        """Logic giống như trong scheduler."""
        if current_time is None:
            now = datetime.now(vietnam_tz)
        else:
            now = current_time
            
        today = now.date()
        
        # Tìm khung giờ tiếp theo trong ngày
        for time_str in schedule_times:
            hour, minute = map(int, time_str.split(':'))
            scheduled_time = vietnam_tz.localize(
                datetime.combine(today, datetime.min.time().replace(hour=hour, minute=minute))
            )
            
            if scheduled_time > now:
                return scheduled_time
        
        # Nếu không có khung giờ nào còn lại trong ngày, lấy khung đầu tiên của ngày mai
        tomorrow = today + timedelta(days=1)
        hour, minute = map(int, schedule_times[0].split(':'))
        next_run = vietnam_tz.localize(
            datetime.combine(tomorrow, datetime.min.time().replace(hour=hour, minute=minute))
        )
        return next_run
    
    # Test cases
    test_cases = [
        # (current_time_str, expected_next_str, description)
        ("2025-10-22 06:00:00", "2025-10-22 07:30:00", "Sáng sớm -> 7:30 cùng ngày"),
        ("2025-10-22 08:00:00", "2025-10-22 15:00:00", "Sau 7:30 -> 15:00 cùng ngày"),
        ("2025-10-22 16:00:00", "2025-10-22 19:00:00", "Sau 15:00 -> 19:00 cùng ngày"),
        ("2025-10-22 20:00:00", "2025-10-23 07:30:00", "Sau 19:00 -> 7:30 ngày mai"),
        ("2025-10-22 07:30:00", "2025-10-22 15:00:00", "Đúng 7:30 -> 15:00 cùng ngày"),
        ("2025-10-22 07:31:00", "2025-10-22 15:00:00", "Sau 7:30 1 phút -> 15:00 cùng ngày"),
    ]
    
    print("🧪 Testing scheduler logic...")
    print("=" * 70)
    
    for current_str, expected_str, description in test_cases:
        # Parse test time
        current_time = vietnam_tz.localize(
            datetime.strptime(current_str, "%Y-%m-%d %H:%M:%S")
        )
        expected_time = vietnam_tz.localize(
            datetime.strptime(expected_str, "%Y-%m-%d %H:%M:%S")
        )
        
        # Test
        result = get_next_run_time(current_time)
        success = result == expected_time
        
        print(f"📅 {description}")
        print(f"   Current:  {current_time.strftime('%Y-%m-%d %H:%M:%S %Z')}")
        print(f"   Expected: {expected_time.strftime('%Y-%m-%d %H:%M:%S %Z')}")
        print(f"   Result:   {result.strftime('%Y-%m-%d %H:%M:%S %Z')}")
        print(f"   Status:   {'✅ PASS' if success else '❌ FAIL'}")
        print()
    
    print("=" * 70)
    print("🎯 Schedule times: 07:30, 15:00, 19:00 (GMT+7)")
    print("📍 Current time in Vietnam:", datetime.now(vietnam_tz).strftime('%Y-%m-%d %H:%M:%S %Z'))
    print("⏰ Next scheduled run:", get_next_run_time().strftime('%Y-%m-%d %H:%M:%S %Z'))


def test_time_tolerance():
    """Test tolerance check logic."""
    vietnam_tz = pytz.timezone('Asia/Ho_Chi_Minh')
    schedule_times = ["07:30", "15:00", "19:00"]
    
    def should_run_now(current_time):
        """Kiểm tra xem có nên chạy báo cáo không."""
        for time_str in schedule_times:
            scheduled_hour, scheduled_minute = map(int, time_str.split(':'))
            scheduled_time = current_time.replace(
                hour=scheduled_hour, 
                minute=scheduled_minute, 
                second=0, 
                microsecond=0
            )
            time_diff = abs((current_time - scheduled_time).total_seconds())
            
            if time_diff <= 300:  # Tolerance 5 phút
                return True, time_str, time_diff
        return False, None, None
    
    # Test cases
    test_times = [
        ("07:28:00", True, "2 phút trước 7:30"),
        ("07:30:00", True, "Đúng 7:30"),
        ("07:32:00", True, "2 phút sau 7:30"),
        ("07:36:00", False, "6 phút sau 7:30 (ngoài tolerance)"),
        ("15:05:00", True, "5 phút sau 15:00"),
        ("15:06:00", False, "6 phút sau 15:00 (ngoài tolerance)"),
        ("19:00:00", True, "Đúng 19:00"),
        ("12:00:00", False, "Giữa trưa (không phải khung giờ)"),
    ]
    
    print("\n🕐 Testing time tolerance logic...")
    print("=" * 70)
    
    base_date = vietnam_tz.localize(datetime(2025, 10, 22))
    
    for time_str, expected, description in test_times:
        hour, minute, second = map(int, time_str.split(':'))
        test_time = base_date.replace(hour=hour, minute=minute, second=second)
        
        should_run, matched_schedule, time_diff = should_run_now(test_time)
        success = should_run == expected
        
        print(f"🕐 {description}")
        print(f"   Time: {test_time.strftime('%H:%M:%S')}")
        print(f"   Should run: {should_run} (expected: {expected})")
        if should_run:
            print(f"   Matched schedule: {matched_schedule}, diff: {time_diff:.0f}s")
        print(f"   Status: {'✅ PASS' if success else '❌ FAIL'}")
        print()


if __name__ == "__main__":
    print("🚀 Testing Fixed Time Scheduler Logic")
    print("=" * 70)
    
    try:
        test_get_next_run_time()
        test_time_tolerance()
        
        print("✅ All tests completed!")
        
    except ImportError as e:
        print(f"❌ Import error: {e}")
        print("Make sure pytz is installed: pip install pytz")
    except Exception as e:
        print(f"❌ Error: {e}")