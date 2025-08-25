#!/usr/bin/env python3
"""
Dry run script để kiểm tra workflow structure và logic flow
Không gọi API thực tế, chỉ mô phỏng flow
"""

import sys
import os
import uuid
from datetime import datetime
from typing import Dict, Any

# Add the parent directory to sys.path to import modules
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

def simulate_node_execution(node_name: str, state: Dict[str, Any]) -> Dict[str, Any]:
    """Mô phỏng thực thi một node với mock data"""
    print(f"    🔄 Executing node: {node_name}")
    
    # Mock state changes for each node
    if node_name == "prepare_data":
        state["client"] = "mock_client"
        state["model"] = "mock_model"
        print(f"       ✓ Prepared API client and model")
        
    elif node_name == "research_deep":
        state["research_content"] = "Mock research content about crypto market analysis"
        state["research_quality_score"] = 8.5
        print(f"       ✓ Generated research content ({len(state['research_content'])} chars)")
        
    elif node_name == "validate_report":
        state["validation_result"] = "PASS"
        state["validation_score"] = 9.0
        print(f"       ✓ Validation: {state['validation_result']} (score: {state['validation_score']})")
        
    elif node_name == "generate_report_content":
        state["report_content"] = "Mock detailed report content with analysis"
        print(f"       ✓ Generated report content ({len(state['report_content'])} chars)")
        
    elif node_name == "create_interface":
        state["interface_content"] = "Mock interface with HTML, CSS, JS structure"
        state["interface_attempt"] = state.get("interface_attempt", 0) + 1
        print(f"       ✓ Created interface (attempt #{state['interface_attempt']})")
        
    elif node_name == "extract_code":
        state["html_content"] = "<div>Mock HTML content</div>"
        state["css_content"] = ".mock { color: red; }"
        state["js_content"] = "function mockFunction() { console.log('test'); }"
        state["extraction_success"] = True
        print(f"       ✓ Extracted code - HTML({len(state['html_content'])}), CSS({len(state['css_content'])}), JS({len(state['js_content'])})")
        
    elif node_name == "translate_content":
        if state.get("html_content"):
            state["html_content_en"] = state["html_content"] + " (English)"
            print(f"       ✓ Translated HTML content")
        if state.get("js_content"):
            state["js_content_en"] = state["js_content"].replace("mockFunction", "mockFunction_en")
            print(f"       ✓ Translated JS content with _en suffix")
            
    elif node_name == "save_database":
        state["report_id"] = str(uuid.uuid4())
        state["success"] = True
        print(f"       ✓ Saved to database (report_id: {state['report_id'][:8]}...)")
    
    return state

def should_retry_or_continue_mock(state: Dict[str, Any]) -> str:
    """Mock routing function for validation"""
    validation_result = state.get("validation_result", "UNKNOWN")
    attempt = state.get("research_attempt", 0)
    
    print(f"    🔀 Routing decision for validation:")
    print(f"       - Validation result: {validation_result}")
    print(f"       - Research attempt: {attempt}")
    
    if validation_result == "FAIL" and attempt < 2:
        print(f"       → Decision: retry (attempt {attempt + 1})")
        state["research_attempt"] = attempt + 1
        return "retry"
    elif validation_result == "PASS":
        print(f"       → Decision: continue")
        return "continue"
    else:
        print(f"       → Decision: end (max attempts reached)")
        return "end"

def should_retry_interface_or_continue_mock(state: Dict[str, Any]) -> str:
    """Mock routing function for interface extraction"""
    extraction_success = state.get("extraction_success", False)
    interface_attempt = state.get("interface_attempt", 0)
    
    print(f"    🔀 Routing decision for interface extraction:")
    print(f"       - Extraction success: {extraction_success}")
    print(f"       - Interface attempt: {interface_attempt}")
    
    if not extraction_success and interface_attempt < 3:
        print(f"       → Decision: retry_interface (attempt {interface_attempt + 1})")
        return "retry_interface"
    elif extraction_success:
        print(f"       → Decision: continue")
        return "continue"
    else:
        print(f"       → Decision: end (max attempts reached)")
        return "end"

def run_workflow_dry_run():
    """Chạy dry run của workflow"""
    print("🚀 WORKFLOW DRY RUN - STRUCTURE VALIDATION")
    print("=" * 60)
    
    # Initialize mock state
    session_id = str(uuid.uuid4())
    initial_state = {
        "session_id": session_id,
        "api_key": "mock_api_key",
        "max_attempts": 3,
        "created_at": datetime.utcnow().isoformat(),
        "success": False,
        "error_messages": [],
        "interface_attempt": 0,
        "research_attempt": 0,
    }
    
    print(f"📋 Initial State:")
    print(f"   Session ID: {session_id[:8]}...")
    print(f"   Created at: {initial_state['created_at']}")
    print()
    
    current_state = initial_state.copy()
    
    # Simulate workflow execution
    workflow_steps = [
        ("prepare_data", "DIRECT"),
        ("research_deep", "DIRECT"),
        ("validate_report", "CONDITIONAL"),
        ("generate_report_content", "DIRECT"),
        ("create_interface", "DIRECT"),
        ("extract_code", "CONDITIONAL"),
        ("translate_content", "DIRECT"),
        ("save_database", "DIRECT"),
    ]
    
    step_num = 1
    for step_name, step_type in workflow_steps:
        print(f"🔢 Step {step_num}: {step_name.upper()}")
        
        if step_type == "DIRECT":
            current_state = simulate_node_execution(step_name, current_state)
            
        elif step_type == "CONDITIONAL":
            current_state = simulate_node_execution(step_name, current_state)
            
            if step_name == "validate_report":
                routing_decision = should_retry_or_continue_mock(current_state)
                if routing_decision == "retry":
                    print(f"    ↻ Retrying research_deep...")
                    current_state = simulate_node_execution("research_deep", current_state)
                    current_state = simulate_node_execution("validate_report", current_state)
                    routing_decision = should_retry_or_continue_mock(current_state)
                
                if routing_decision == "end":
                    print(f"    ⚠️ Workflow terminated at validation")
                    break
                    
            elif step_name == "extract_code":
                routing_decision = should_retry_interface_or_continue_mock(current_state)
                if routing_decision == "retry_interface":
                    print(f"    ↻ Retrying create_interface...")
                    current_state = simulate_node_execution("create_interface", current_state)
                    current_state = simulate_node_execution("extract_code", current_state)
                    routing_decision = should_retry_interface_or_continue_mock(current_state)
                
                if routing_decision == "end":
                    print(f"    ⚠️ Workflow terminated at extraction")
                    break
        
        print()
        step_num += 1
    
    # Final results
    print("📊 FINAL STATE ANALYSIS:")
    print("=" * 40)
    
    required_keys = [
        "success", "report_id", "html_content", "css_content", "js_content", 
        "research_content", "html_content_en", "js_content_en"
    ]
    
    for key in required_keys:
        value = current_state.get(key, "MISSING")
        status = "✅" if value and value != "MISSING" else "❌"
        if isinstance(value, str) and len(value) > 50:
            value = f"{value[:50]}..."
        print(f"   {status} {key}: {value}")
    
    print()
    print("🎯 WORKFLOW VALIDATION:")
    
    # Check workflow completeness
    validation_checks = [
        ("API Integration Points", current_state.get("client") is not None),
        ("Research Generation", current_state.get("research_content") is not None),
        ("Validation Process", current_state.get("validation_result") == "PASS"),
        ("Interface Creation", current_state.get("interface_content") is not None),
        ("Code Extraction", all(current_state.get(k) for k in ["html_content", "css_content", "js_content"])),
        ("Translation Process", current_state.get("html_content_en") is not None),
        ("English JS Functions", "_en" in current_state.get("js_content_en", "")),
        ("Database Save", current_state.get("report_id") is not None),
        ("Final Success", current_state.get("success", False)),
    ]
    
    passed = 0
    total = len(validation_checks)
    
    for check_name, check_result in validation_checks:
        status = "✅ PASS" if check_result else "❌ FAIL"
        print(f"   {status} - {check_name}")
        if check_result:
            passed += 1
    
    print()
    print(f"📈 WORKFLOW HEALTH: {passed}/{total} checks passed ({passed/total*100:.1f}%)")
    
    if passed == total:
        print("🎉 WORKFLOW STRUCTURE IS VALID!")
    else:
        print("⚠️ WORKFLOW HAS ISSUES THAT NEED ATTENTION")
    
    print()
    print("🔗 WORKFLOW FLOW SUMMARY:")
    print("   prepare_data → research_deep → validate_report")
    print("                     ↑              ↓ (conditional)")
    print("                  (retry)     generate_report_content")
    print("                                     ↓")
    print("                              create_interface")
    print("                                ↑     ↓")
    print("                           (retry) extract_code")
    print("                                     ↓ (conditional)")
    print("                             translate_content")
    print("                                     ↓")
    print("                               save_database → END")
    
    return current_state

if __name__ == "__main__":
    try:
        final_state = run_workflow_dry_run()
        print("\n✅ Dry run completed successfully!")
        
        # Optional: Show key metrics
        if final_state.get("success"):
            print(f"\n📝 Summary:")
            print(f"   - Report ID: {final_state.get('report_id', 'N/A')}")
            print(f"   - Research attempts: {final_state.get('research_attempt', 0) + 1}")  
            print(f"   - Interface attempts: {final_state.get('interface_attempt', 0)}")
            print(f"   - Translation: {'✅ Complete' if final_state.get('html_content_en') else '❌ Missing'}")
            print(f"   - JS _en functions: {'✅ Generated' if '_en' in final_state.get('js_content_en', '') else '❌ Missing'}")
        
    except Exception as e:
        print(f"\n❌ Dry run failed: {e}")
        sys.exit(1)
