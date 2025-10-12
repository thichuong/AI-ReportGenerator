"""
Base types và utilities cho workflow
"""
import os
import re
import json
import asyncio
from datetime import datetime, timezone
from typing import TypedDict, Optional, List
from google import genai
from google.genai import types
import redis


class ReportState(TypedDict):
    """State schema cho report generation workflow"""
    # Session tracking
    session_id: str
    
    # Input parameters
    api_key: str
    max_attempts: int
    
    # File paths
    research_analysis_prompt_path: Optional[str]
    data_validation_prompt_path: Optional[str]
    create_report_prompt_path: Optional[str]
    
    # Processing state
    research_analysis_prompt: Optional[str]
    data_validation_prompt: Optional[str]
    create_report_prompt: Optional[str]
    research_content: Optional[str]
    report_content: Optional[str]
    validation_result: Optional[str]
    interface_content: Optional[str]
    realtime_data: Optional[dict]  # Cache for real-time dashboard data
    
    # Output
    html_content: Optional[str]
    css_content: Optional[str]
    js_content: Optional[str]
    html_content_en: Optional[str]  # HTML content translated to English
    js_content_en: Optional[str] # JS content translated to English
    report_id: Optional[int]
    
    # Control flow
    current_attempt: int
    error_messages: List[str]
    success: bool
    
    # Component-specific attempt counters (for workflow v2)
    html_attempt: Optional[int]
    js_attempt: Optional[int]
    css_attempt: Optional[int]
    interface_attempt: Optional[int]  # For backward compatibility
    
    # Timestamps
    created_at: Optional[str]
    
    # Gemini client
    client: Optional[object]
    model: str


def read_prompt_file(file_path):
    """Đọc nội dung từ tệp prompt."""
    try:
        # Nếu chỉ là tên file, tìm trong thư mục create_report
        if not os.path.isabs(file_path) and not os.path.dirname(file_path):
            current_dir = os.path.dirname(__file__)
            project_root = os.path.abspath(os.path.join(current_dir, '..', '..', '..'))
            file_path = os.path.join(project_root, 'create_report', file_path)
        
        # Kiểm tra file tồn tại
        if not os.path.exists(file_path):
            print(f"Lỗi: File không tồn tại tại '{file_path}'")
            return None
            
        with open(file_path, 'r', encoding='utf-8') as f:
            template = f.read()
            
            # Kiểm tra nội dung template
            if not template or not isinstance(template, str):
                print(f"Lỗi: Nội dung file trống hoặc không hợp lệ tại '{file_path}'")
                return None
            
            # Đọc toàn bộ nội dung file create_report/colors.css
            current_dir = os.path.dirname(__file__)
            project_root = os.path.abspath(os.path.join(current_dir, '..', '..', '..'))
            colors = os.path.join(project_root, 'create_report', 'colors.css')
            
            # Kiểm tra file colors.css tồn tại
            if not os.path.exists(colors):
                print(f"Cảnh báo: File colors.css không tồn tại tại '{colors}' - sử dụng giá trị mặc định")
                colors_content = ""
            else:
                try:
                    with open(colors, 'r', encoding='utf-8') as f:
                        colors_content = f.read()
                        
                        if colors_content:
                            # Lấy nội dung :root trong file colors.css
                            colors_match = re.search(r':root\s*{([^}]+)}', colors_content, re.DOTALL)
                            if colors_match:
                                colors_content = colors_match.group(1).strip()
                            else:
                                print("Cảnh báo: Không tìm thấy nội dung :root trong file colors.css")
                                colors_content = ""
                        else:
                            colors_content = ""
                except Exception as e:
                    print(f"Lỗi khi đọc file colors.css: {e}")
                    colors_content = ""
                
            # Thay thế biến trong template
            prompt = template.replace("{{ @css_root }}", colors_content)
            return prompt
            
    except FileNotFoundError:
        print(f"Lỗi: Không tìm thấy tệp prompt tại '{file_path}'")
        return None
    except Exception as e:
        print(f"Lỗi khi đọc file '{file_path}': {e}")
        return None


def replace_date_placeholders(prompt_text):
    """Thay thế các placeholder về ngày tháng năm trong prompt."""
    now = datetime.now(timezone.utc)
    
    prompt_text = prompt_text.replace("<<@day>>", str(now.day))
    prompt_text = prompt_text.replace("<<@month>>", str(now.month))
    prompt_text = prompt_text.replace("<<@year>>", str(now.year))
    
    return prompt_text


def get_prompt_from_env(prompt_name: str) -> Optional[str]:
    """
    Lấy prompt từ biến môi trường.
    
    Args:
        prompt_name: Tên của biến môi trường prompt (ví dụ: 'prompt_create_html')
        
    Returns:
        Nội dung prompt hoặc None nếu không tìm thấy
    """
    prompt_content = os.environ.get(prompt_name)
    
    if not prompt_content:
        print(f"Cảnh báo: Không tìm thấy prompt '{prompt_name}' trong biến môi trường")
        return None
        
    # Thay thế placeholder CSS nếu có
    if "{{ @css_root }}" in prompt_content:
        # Đọc colors.css từ app/static/css
        current_dir = os.path.dirname(__file__)
        project_root = os.path.abspath(os.path.join(current_dir, '..', '..', '..'))
        colors_path = os.path.join(project_root, 'app', 'static', 'css', 'colors.css')
        
        colors_content = ""
        if os.path.exists(colors_path):
            try:
                with open(colors_path, 'r', encoding='utf-8') as f:
                    colors_file_content = f.read()
                    # Lấy nội dung :root
                    colors_match = re.search(r':root\s*{([^}]+)}', colors_file_content, re.DOTALL)
                    if colors_match:
                        colors_content = colors_match.group(1).strip()
                    else:
                        print("Cảnh báo: Không tìm thấy nội dung :root trong colors.css")
            except Exception as e:
                print(f"Lỗi khi đọc colors.css: {e}")
        else:
            print(f"Cảnh báo: colors.css không tồn tại tại {colors_path}")
        
        prompt_content = prompt_content.replace("{{ @css_root }}", colors_content)
    
    return prompt_content


def extract_code_blocks(response_text):
    """Trích xuất các khối mã nguồn (html, css, js) từ phản hồi của Gemini."""
    # Kiểm tra input
    if not response_text or not isinstance(response_text, str):
        print("Cảnh báo: response_text là None hoặc không phải string")
        return {
            "html": "",
            "css": "/* Lỗi: Không có nội dung phản hồi */",
            "js": "// Lỗi: Không có nội dung phản hồi",
            "success": False
        }
    
    html_match = re.search(r"```html(.*?)```", response_text, re.DOTALL)
    css_match = re.search(r"```css(.*?)```", response_text, re.DOTALL)
    js_match = re.search(r"```javascript(.*?)```", response_text, re.DOTALL)

    if not js_match:
        js_match = re.search(r"```js(.*?)```", response_text, re.DOTALL)

    # Kiểm tra xem có ít nhất HTML hoặc có nội dung hữu ích
    html_content = html_match.group(1).strip() if html_match else ""
    css_content = css_match.group(1).strip() if css_match else "/* Lỗi: Không trích xuất được CSS */"
    js_content = js_match.group(1).strip() if js_match else "// Lỗi: Không trích xuất được JS"
    
    # Xác định trạng thái thành công
    # Coi là thành công nếu có HTML hoặc có ít nhất 2 trong 3 thành phần
    has_html = bool(html_content)
    has_css = css_match is not None
    has_js = js_match is not None
    
    # Hoặc kiểm tra xem có HTML tags trong response không (trường hợp không có code blocks)
    has_html_tags = bool(re.search(r'<html|<!doctype|<div|<body|<head', response_text, re.IGNORECASE))
    
    success = has_html or has_html_tags or (has_css and has_js)

    return {
        "html": html_content,
        "css": css_content,
        "js": js_content,
        "success": success
    }


def check_report_validation(report_text):
    """
    Kiểm tra kết quả validation của báo cáo.
    
    Returns:
        str: 'PASS', 'FAIL', hoặc 'UNKNOWN'
    """
    # Kiểm tra input
    if not report_text or not isinstance(report_text, str):
        print("Cảnh báo: report_text là None hoặc không phải string")
        return 'UNKNOWN'
    
    # Tìm kết quả kiểm tra cuối cùng
    pass_pattern = re.search(r"KẾT QUẢ KIỂM TRA:\s*PASS", report_text, re.IGNORECASE)
    fail_pattern = re.search(r"KẾT QUẢ KIỂM TRA:\s*FAIL", report_text, re.IGNORECASE)
    
    if pass_pattern:
        return 'PASS'
    elif fail_pattern:
        return 'FAIL'
    else:
        return 'UNKNOWN'


def get_realtime_dashboard_data():
    """Lấy dữ liệu crypto thời gian thực từ Redis với key 'latest_market_data'"""
    max_retries = 3
    retry_delay = 2  # giây
    
    for attempt in range(max_retries):
        try:
            # Lấy dữ liệu từ Redis
            redis_url = os.getenv('REDIS_URL')
            if not redis_url:
                print("Warning: REDIS_URL not set, cannot fetch market data")
                return None
                
            r = redis.Redis.from_url(redis_url)
            cached_data = r.get('latest_market_data')
            
            if cached_data:
                # Parse JSON data
                try:
                    full_data = json.loads(cached_data.decode('utf-8'))
                    
                    # Lấy dữ liệu crypto đầy đủ hơn
                    crypto_data = {}
                    
                    # Mapping các trường crypto - Mở rộng để lấy nhiều chỉ số hơn
                    field_mappings = {
                        # BTC metrics
                        'btc_price': ['btc_price', 'btc_price_usd', 'btc', 'bitcoin'],
                        'btc_change_24h': ['btc_change_24h', 'btc_24h_change', 'bitcoin_change_24h'],
                        'btc_rsi_14': ['btc_rsi_14', 'rsi_14', 'btc_rsi', 'rsi'],
                        
                        # ETH metrics
                        'eth_price': ['eth_price', 'eth_price_usd', 'eth', 'ethereum'],
                        'eth_change_24h': ['eth_change_24h', 'eth_24h_change', 'ethereum_change_24h'],
                        
                        # SOL metrics
                        'sol_price': ['sol_price', 'sol_price_usd', 'sol', 'solana'],
                        'sol_change_24h': ['sol_change_24h', 'sol_24h_change', 'solana_change_24h'],
                        
                        # XRP metrics
                        'xrp_price': ['xrp_price', 'xrp_price_usd', 'xrp'],
                        'xrp_change_24h': ['xrp_change_24h', 'xrp_24h_change'],
                        
                        # ADA metrics
                        'ada_price': ['ada_price', 'ada_price_usd', 'ada', 'cardano'],
                        'ada_change_24h': ['ada_change_24h', 'ada_24h_change', 'cardano_change_24h'],
                        
                        # LINK metrics
                        'link_price': ['link_price', 'link_price_usd', 'link', 'chainlink'],
                        'link_change_24h': ['link_change_24h', 'link_24h_change', 'chainlink_change_24h'],
                        
                        # BNB metrics
                        'bnb_price': ['bnb_price', 'bnb_price_usd', 'bnb', 'binance_coin'],
                        'bnb_change_24h': ['bnb_change_24h', 'bnb_24h_change'],
                        
                        # Market metrics
                        'market_cap': ['market_cap', 'market_cap_usd', 'total_market_cap'],
                        'volume_24h': ['volume_24h', 'volume_24h_usd', 'total_volume_24h'],
                        'market_cap_change_24h': ['market_cap_change_percentage_24h_usd', 'market_cap_change_24h', 'total_market_cap_change_24h'],
                        
                        # Dominance metrics
                        'btc_dominance': ['btc_dominance', 'btc_market_cap_percentage', 'bitcoin_dominance'],
                        'eth_dominance': ['eth_dominance', 'eth_market_cap_percentage', 'ethereum_dominance'],
                        
                        # Sentiment indicators
                        'fear_greed_index': ['fear_greed_index', 'fng_value', 'fear_and_greed_index'],
                        
                        # Timestamp & source
                        'timestamp': ['timestamp', 'last_updated', 'updated_at'],
                        'source': ['source', 'data_source', 'normalized_by'],
                    }
                    
                    # Tìm và extract từng trường
                    for target_field, possible_keys in field_mappings.items():
                        for key in possible_keys:
                            if key in full_data:
                                crypto_data[target_field] = full_data[key]
                                break
                    
                    # Lấy thêm data_sources nếu có
                    if 'data_sources' in full_data:
                        crypto_data['data_sources'] = full_data['data_sources']
                    
                    # Lấy thêm partial_failure flag nếu có
                    if 'partial_failure' in full_data:
                        crypto_data['partial_failure'] = full_data['partial_failure']
                    
                    # Lấy thêm fetch_duration_ms nếu có
                    if 'fetch_duration_ms' in full_data:
                        crypto_data['fetch_duration_ms'] = full_data['fetch_duration_ms']
                    
                    # Thêm data_source
                    crypto_data["data_source"] = "real_time"
                    
                    # In giá trị ra - Mở rộng để hiển thị nhiều chỉ số hơn
                    print("=== CRYPTO DATA FROM REDIS ===")
                    print(f"BTC Price: {crypto_data.get('btc_price', 'N/A')}")
                    print(f"BTC Change 24h: {crypto_data.get('btc_change_24h', 'N/A')}%")
                    print(f"BTC RSI 14: {crypto_data.get('btc_rsi_14', 'N/A')}")
                    print(f"ETH Price: {crypto_data.get('eth_price', 'N/A')}")
                    print(f"ETH Change 24h: {crypto_data.get('eth_change_24h', 'N/A')}%")
                    print(f"SOL Price: {crypto_data.get('sol_price', 'N/A')}")
                    print(f"XRP Price: {crypto_data.get('xrp_price', 'N/A')}")
                    print(f"ADA Price: {crypto_data.get('ada_price', 'N/A')}")
                    print(f"LINK Price: {crypto_data.get('link_price', 'N/A')}")
                    print(f"BNB Price: {crypto_data.get('bnb_price', 'N/A')}")
                    print(f"Market Cap: {crypto_data.get('market_cap', 'N/A')}")
                    print(f"Market Cap Change 24h: {crypto_data.get('market_cap_change_24h', 'N/A')}%")
                    print(f"Volume 24h: {crypto_data.get('volume_24h', 'N/A')}")
                    print(f"BTC Dominance: {crypto_data.get('btc_dominance', 'N/A')}%")
                    print(f"ETH Dominance: {crypto_data.get('eth_dominance', 'N/A')}%")
                    print(f"Fear & Greed Index: {crypto_data.get('fear_greed_index', 'N/A')}")
                    print(f"Timestamp: {crypto_data.get('timestamp', 'N/A')}")
                    print("============================")
                    
                    print(f"Successfully retrieved crypto data from Redis on attempt {attempt + 1}: {len(crypto_data)} fields")
                    return crypto_data
                    
                except json.JSONDecodeError as e:
                    print(f"Error parsing cached market data: {e}")
                    return None
            else:
                if attempt < max_retries - 1:
                    print(f"No cached market data found in Redis (attempt {attempt + 1}/{max_retries}), retrying in {retry_delay} seconds...")
                    asyncio.run(asyncio.sleep(retry_delay))
                else:
                    print(f"No cached market data found in Redis after {max_retries} attempts")
                    return None
                    
        except Exception as e:
            print(f"Error getting cached crypto data from Redis: {e}")
            return None
