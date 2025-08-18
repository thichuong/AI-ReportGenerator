import os
import time
from .api_client import fetch_json

# Global variables for rate limiting
_last_request_time = 0
_min_request_interval = 60  # Minimum 60 seconds between requests

# Base URL template for TAAPI RSI endpoint
BASE_RSI_URL_TEMPLATE = "https://api.taapi.io/rsi?secret={secret}&exchange=binance&symbol=BTC/USDT&interval=1d"

def get_btc_rsi():
    """Lấy chỉ số RSI của Bitcoin từ TAAPI.IO với rate limiting và backup cache."""
    global _last_request_time, _min_request_interval
    
    api_key = os.getenv('TAAPI_SECRET')
    if not api_key:
        return None, "TAAPI_SECRET không được cấu hình", 500
        
    api_url = BASE_RSI_URL_TEMPLATE.format(secret=api_key)

    # Kiểm tra rate limiting
    current_time = time.time()
    time_since_last_request = current_time - _last_request_time
    
    if time_since_last_request < _min_request_interval:
        # Không sử dụng cache/backup nữa — trả về thông báo rate limit trực tiếp
        time_to_wait = _min_request_interval - time_since_last_request
        return None, f"Rate limit: phải chờ {int(time_to_wait)} giây nữa", 429

    _last_request_time = current_time
    json_data, error, status_code = fetch_json(api_url, timeout=3)

    if error:
        # Nếu gặp rate limit, tăng thời gian chờ, trả về lỗi (không dùng cache)
        if status_code == 429:
            _min_request_interval = min(_min_request_interval * 2, 300)  # Tối đa 5 phút
            return None, f"Rate limit from TAAPI: {error}", status_code
        return None, f"Lỗi khi gọi TAAPI: {error}", status_code

    try:
        rsi_value = json_data.get('value')
        if rsi_value is None:
            raise KeyError("Không tìm thấy 'value' trong phản hồi của TAAPI.")
        data = {'bitcoin_rsi_14': rsi_value}
        
    # Cache/backup đã bị bỏ — không lưu local
    # Reset interval sau khi request thành công
        _min_request_interval = 60
        return data, None, 200
    except (AttributeError, KeyError) as e:
        return None, f"Lỗi xử lý dữ liệu RSI từ TAAPI: {e}", 500