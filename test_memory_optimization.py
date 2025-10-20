#!/usr/bin/env python3
"""
Script kiểm tra memory usage của các schedulers và workflow
Sử dụng psutil để monitor memory consumption
"""
import os
import sys
import time
import psutil
import asyncio
from datetime import datetime

# Add parent directory to path
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))


def get_memory_usage():
    """Lấy thông tin memory usage hiện tại"""
    process = psutil.Process(os.getpid())
    mem_info = process.memory_info()
    return {
        'rss': mem_info.rss / 1024 / 1024,  # MB
        'vms': mem_info.vms / 1024 / 1024,  # MB
        'percent': process.memory_percent()
    }


def print_memory_stats(label=""):
    """In ra thống kê memory"""
    mem = get_memory_usage()
    timestamp = datetime.now().strftime("%H:%M:%S")
    print(f"[{timestamp}] {label}")
    print(f"  RSS: {mem['rss']:.2f} MB")
    print(f"  VMS: {mem['vms']:.2f} MB")
    print(f"  Percent: {mem['percent']:.2f}%")


async def test_market_indices_memory():
    """Test memory usage của market indices scheduler"""
    print("\n" + "="*60)
    print("TEST: Market Indices Fetch Memory Usage")
    print("="*60)
    
    from app.routers.market import fetch_and_store_market_indices
    
    print_memory_stats("Before fetching market indices")
    
    # Run multiple times to check for memory leaks
    for i in range(5):
        print(f"\n--- Iteration {i+1}/5 ---")
        try:
            result = await fetch_and_store_market_indices()
            print(f"✅ Fetch successful: {result.get('success', False)}")
        except Exception as e:
            print(f"❌ Error: {e}")
        
        print_memory_stats(f"After iteration {i+1}")
        time.sleep(1)
    
    print("\n" + "="*60)
    print("Market Indices Test Complete")
    print("="*60)


def test_scheduler_memory():
    """Test memory usage của report scheduler"""
    print("\n" + "="*60)
    print("TEST: Report Scheduler Memory Usage (Simulated)")
    print("="*60)
    
    print_memory_stats("Before scheduler simulation")
    
    # Simulate what happens in scheduler
    from app.services.auto_report_scheduler import generate_auto_research_report
    
    api_key = os.getenv('GEMINI_API_KEY')
    if not api_key:
        print("⚠️ GEMINI_API_KEY not set, skipping scheduler test")
        return
    
    print("\nSimulating report generation (this may take a while)...")
    print_memory_stats("Before report generation")
    
    try:
        result = generate_auto_research_report(api_key, max_attempts=1)
        print(f"Result: {result}")
    except Exception as e:
        print(f"❌ Error: {e}")
    
    print_memory_stats("After report generation")
    
    # Force garbage collection
    import gc
    gc.collect()
    print_memory_stats("After garbage collection")
    
    print("\n" + "="*60)
    print("Scheduler Test Complete")
    print("="*60)


async def test_database_save_memory():
    """Test memory usage của database save operation"""
    print("\n" + "="*60)
    print("TEST: Database Save Memory Usage (Mock)")
    print("="*60)
    
    print_memory_stats("Before creating large state")
    
    # Create a mock state with large content
    large_html = "<html>" + "x" * 1024 * 1024 + "</html>"  # 1MB
    large_css = "body { " + "color: red; " * 10000 + "}"
    large_js = "console.log('" + "test" * 100000 + "');"
    
    state = {
        "html_content": large_html,
        "css_content": large_css,
        "js_content": large_js,
        "html_content_en": large_html,
        "js_content_en": large_js,
        "session_id": "test_session"
    }
    
    print_memory_stats("After creating large state")
    print(f"State size: ~{(len(large_html) + len(large_css) + len(large_js)) / 1024 / 1024:.2f} MB")
    
    # Simulate cleanup
    large_fields = ["html_content", "css_content", "js_content", 
                   "html_content_en", "js_content_en"]
    for field in large_fields:
        if field in state:
            del state[field]
    
    del large_html, large_css, large_js
    
    import gc
    gc.collect()
    
    print_memory_stats("After cleanup and gc.collect()")
    
    print("\n" + "="*60)
    print("Database Save Test Complete")
    print("="*60)


def test_memory_leak_detection():
    """Test để phát hiện memory leak"""
    print("\n" + "="*60)
    print("TEST: Memory Leak Detection")
    print("="*60)
    
    iterations = 10
    memory_readings = []
    
    for i in range(iterations):
        # Create and destroy large objects
        data = ["x" * 1024 * 100 for _ in range(100)]  # ~10MB
        
        mem = get_memory_usage()
        memory_readings.append(mem['rss'])
        
        del data
        
        import gc
        gc.collect()
        
        print(f"Iteration {i+1}/{iterations}: {mem['rss']:.2f} MB")
        time.sleep(0.5)
    
    # Analyze trend
    print(f"\nMemory readings: {[f'{m:.2f}' for m in memory_readings]}")
    
    first_half_avg = sum(memory_readings[:5]) / 5
    second_half_avg = sum(memory_readings[5:]) / 5
    
    print(f"\nFirst half average: {first_half_avg:.2f} MB")
    print(f"Second half average: {second_half_avg:.2f} MB")
    
    growth = second_half_avg - first_half_avg
    growth_percent = (growth / first_half_avg) * 100
    
    print(f"\nMemory growth: {growth:.2f} MB ({growth_percent:.2f}%)")
    
    if growth_percent > 10:
        print("⚠️ WARNING: Potential memory leak detected!")
    else:
        print("✅ Memory appears stable")
    
    print("\n" + "="*60)
    print("Memory Leak Detection Complete")
    print("="*60)


async def main():
    """Chạy tất cả các tests"""
    print("\n" + "="*60)
    print("MEMORY OPTIMIZATION TEST SUITE")
    print("="*60)
    print(f"Start time: {datetime.now()}")
    
    initial_mem = get_memory_usage()
    print(f"\nInitial memory: {initial_mem['rss']:.2f} MB")
    
    try:
        # Test 1: Market indices
        await test_market_indices_memory()
        
        # Test 2: Database save (mock)
        await test_database_save_memory()
        
        # Test 3: Memory leak detection
        test_memory_leak_detection()
        
        # Test 4: Scheduler (optional, commented out as it takes time)
        # test_scheduler_memory()
        
    except KeyboardInterrupt:
        print("\n\n⚠️ Tests interrupted by user")
    except Exception as e:
        print(f"\n\n❌ Error during tests: {e}")
        import traceback
        traceback.print_exc()
    
    final_mem = get_memory_usage()
    print(f"\n\nFinal memory: {final_mem['rss']:.2f} MB")
    print(f"Memory difference: {final_mem['rss'] - initial_mem['rss']:.2f} MB")
    
    print("\n" + "="*60)
    print("ALL TESTS COMPLETE")
    print("="*60)


if __name__ == "__main__":
    # Check if psutil is installed
    try:
        import psutil
    except ImportError:
        print("❌ psutil is not installed. Install it with: pip install psutil")
        sys.exit(1)
    
    # Load environment variables
    from dotenv import load_dotenv
    load_dotenv()
    
    # Run tests
    asyncio.run(main())
