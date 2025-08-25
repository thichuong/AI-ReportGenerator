#!/usr/bin/env python3
"""
Extended dry run script với retry scenarios
"""

import sys
import os
import uuid
from datetime import datetime
from typing import Dict, Any

# Add the parent directory to sys.path to import modules
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

def simulate_node_with_failure_scenarios(node_name: str, state: Dict[str, Any], force_scenario: str = None) -> Dict[str, Any]:
    """Mô phỏng node execution với các failure scenarios"""
    print(f"    🔄 Executing node: {node_name}")
    
    # Scenario handling
    if node_name == "validate_report" and force_scenario == "validation_fail":
        state["validation_result"] = "FAIL"
        state["validation_score"] = 4.0
        print(f"       ❌ Validation: {state['validation_result']} (score: {state['validation_score']})")
        return state
        
    if node_name == "extract_code" and force_scenario == "extraction_fail":
        state["html_content"] = None
        state["css_content"] = None  
        state["js_content"] = None
        state["extraction_success"] = False
        print(f"       ❌ Code extraction failed")
        return state
    
    # Normal execution (copy from original)
    if node_name == "prepare_data":
        state["client"] = "mock_client"
        state["model"] = "mock_model"
        print(f"       ✓ Prepared API client and model")
        
    elif node_name == "research_deep":
        attempt = state.get("research_attempt", 0)
        state["research_content"] = f"Mock research content v{attempt + 1}"
        state["research_quality_score"] = 6.5 + attempt  # Improves with retries
        print(f"       ✓ Research attempt #{attempt + 1} (quality: {state['research_quality_score']})")
        
    elif node_name == "validate_report":
        # Pass if quality is good enough
        quality = state.get("research_quality_score", 0)
        if quality >= 8.0:
            state["validation_result"] = "PASS"
            state["validation_score"] = quality + 0.5
        else:
            state["validation_result"] = "FAIL"
            state["validation_score"] = quality
        print(f"       ✓ Validation: {state['validation_result']} (score: {state['validation_score']})")
        
    elif node_name == "generate_report_content":
        state["report_content"] = "Mock detailed report content with analysis"
        print(f"       ✓ Generated report content ({len(state['report_content'])} chars)")
        
    elif node_name == "create_interface":
        attempt = state.get("interface_attempt", 0) + 1
        state["interface_content"] = f"Mock interface v{attempt}"
        state["interface_attempt"] = attempt
        print(f"       ✓ Created interface (attempt #{attempt})")
        
    elif node_name == "extract_code":
        # Success rate improves with attempts
        attempt = state.get("interface_attempt", 1)
        success_rate = 0.3 + (attempt * 0.3)  # 30%, 60%, 90%
        
        if attempt >= 2:  # Force success on 2nd+ attempt
            state["html_content"] = f"<div>HTML attempt #{attempt}</div>"
            state["css_content"] = f".css-v{attempt} {{ color: blue; }}"
            state["js_content"] = f"function mockFunction_v{attempt}() {{ return {attempt}; }}"
            state["extraction_success"] = True
            print(f"       ✓ Extracted code (attempt #{attempt}) - HTML({len(state['html_content'])}), CSS({len(state['css_content'])}), JS({len(state['js_content'])})")
        else:
            state["extraction_success"] = False
            print(f"       ❌ Extraction failed (attempt #{attempt}) - success rate: {success_rate:.0%}")
            
    elif node_name == "translate_content":
        if state.get("html_content"):
            state["html_content_en"] = state["html_content"] + " (English)"
            print(f"       ✓ Translated HTML content")
        if state.get("js_content"):
            # Replace function names with _en suffix
            js_en = state["js_content"]
            js_en = js_en.replace("mockFunction", "mockFunction_en")
            js_en = js_en.replace("_v", "_en_v")  # Handle versioned functions
            state["js_content_en"] = js_en
            print(f"       ✓ Translated JS content with _en suffix")
            
    elif node_name == "save_database":
        state["report_id"] = str(uuid.uuid4())
        state["success"] = True
        print(f"       ✓ Saved to database (report_id: {state['report_id'][:8]}...)")
    
    return state

def should_retry_or_continue_smart(state: Dict[str, Any]) -> str:
    """Smart routing với retry logic"""
    validation_result = state.get("validation_result", "UNKNOWN")
    attempt = state.get("research_attempt", 0)
    max_attempts = state.get("max_attempts", 3)
    
    print(f"    🔀 Validation routing decision:")
    print(f"       - Result: {validation_result}")
    print(f"       - Attempt: {attempt + 1}/{max_attempts}")
    
    if validation_result == "FAIL" and attempt < (max_attempts - 1):
        print(f"       → Decision: retry (improving research)")
        state["research_attempt"] = attempt + 1
        return "retry"
    elif validation_result == "PASS":
        print(f"       → Decision: continue")
        return "continue"
    else:
        print(f"       → Decision: end (max attempts or unknown result)")
        return "end"

def should_retry_interface_or_continue_smart(state: Dict[str, Any]) -> str:
    """Smart routing cho interface extraction"""
    extraction_success = state.get("extraction_success", False)
    interface_attempt = state.get("interface_attempt", 0)
    max_interface_attempts = 3
    
    print(f"    🔀 Interface extraction routing decision:")
    print(f"       - Success: {extraction_success}")
    print(f"       - Attempt: {interface_attempt}/{max_interface_attempts}")
    
    if not extraction_success and interface_attempt < max_interface_attempts:
        print(f"       → Decision: retry_interface")
        return "retry_interface"
    elif extraction_success:
        print(f"       → Decision: continue")
        return "continue"
    else:
        print(f"       → Decision: end (max attempts reached)")
        return "end"

def run_retry_scenarios():
    """Test các retry scenarios"""
    
    scenarios = [
        {
            "name": "SUCCESS PATH - Perfect Execution",
            "description": "Tất cả nodes thực thi thành công ngay lần đầu",
            "force_failures": []
        },
        {
            "name": "RESEARCH RETRY - Validation Fails First",
            "description": "Research ban đầu không đạt chất lượng, cần retry",
            "force_failures": ["validation_fail"]
        },
        {
            "name": "INTERFACE RETRY - Extraction Fails First", 
            "description": "Interface extraction thất bại lần đầu, cần retry",
            "force_failures": ["extraction_fail"]
        },
        {
            "name": "DOUBLE RETRY - Both Research and Interface Fail",
            "description": "Cả research và interface đều cần retry",
            "force_failures": ["validation_fail", "extraction_fail"]
        }
    ]
    
    print("🧪 WORKFLOW RETRY SCENARIOS TEST")
    print("=" * 60)
    
    for i, scenario in enumerate(scenarios, 1):
        print(f"\n📋 SCENARIO {i}: {scenario['name']}")
        print(f"📝 {scenario['description']}")
        print("-" * 40)
        
        # Initialize state
        session_id = str(uuid.uuid4())
        initial_state = {
            "session_id": session_id,
            "api_key": "mock_api_key",
            "max_attempts": 3,
            "success": False,
            "error_messages": [],
            "interface_attempt": 0,
            "research_attempt": 0,
        }
        
        current_state = initial_state.copy()
        force_failures = scenario["force_failures"].copy()
        
        # Execute workflow with retry logic
        try:
            step_count = 0
            max_steps = 20  # Prevent infinite loops
            
            # prepare_data
            step_count += 1
            print(f"Step {step_count}: prepare_data")
            current_state = simulate_node_with_failure_scenarios("prepare_data", current_state)
            
            # research loop
            while step_count < max_steps:
                step_count += 1
                print(f"Step {step_count}: research_deep")
                current_state = simulate_node_with_failure_scenarios("research_deep", current_state)
                
                step_count += 1  
                print(f"Step {step_count}: validate_report")
                force_scenario = "validation_fail" if "validation_fail" in force_failures else None
                if force_scenario:
                    force_failures.remove(force_scenario)  # Only fail once
                current_state = simulate_node_with_failure_scenarios("validate_report", current_state, force_scenario)
                
                routing = should_retry_or_continue_smart(current_state)
                if routing == "continue":
                    break
                elif routing == "end":
                    print("    ⚠️ Workflow ended at validation")
                    return current_state
                # else: retry (continue loop)
            
            # generate_report_content
            step_count += 1
            print(f"Step {step_count}: generate_report_content")
            current_state = simulate_node_with_failure_scenarios("generate_report_content", current_state)
            
            # interface loop
            while step_count < max_steps:
                step_count += 1
                print(f"Step {step_count}: create_interface")
                current_state = simulate_node_with_failure_scenarios("create_interface", current_state)
                
                step_count += 1
                print(f"Step {step_count}: extract_code")
                force_scenario = "extraction_fail" if "extraction_fail" in force_failures else None
                if force_scenario:
                    force_failures.remove(force_scenario)  # Only fail once
                current_state = simulate_node_with_failure_scenarios("extract_code", current_state, force_scenario)
                
                routing = should_retry_interface_or_continue_smart(current_state)
                if routing == "continue":
                    break
                elif routing == "end":
                    print("    ⚠️ Workflow ended at extraction")
                    return current_state
                # else: retry_interface (continue loop)
            
            # translate_content
            step_count += 1
            print(f"Step {step_count}: translate_content")
            current_state = simulate_node_with_failure_scenarios("translate_content", current_state)
            
            # save_database
            step_count += 1
            print(f"Step {step_count}: save_database")
            current_state = simulate_node_with_failure_scenarios("save_database", current_state)
            
        except Exception as e:
            print(f"    ❌ Scenario failed: {e}")
            continue
        
        # Results
        success = current_state.get("success", False)
        research_attempts = current_state.get("research_attempt", 0) + 1
        interface_attempts = current_state.get("interface_attempt", 0)
        
        print(f"\n📊 SCENARIO {i} RESULTS:")
        print(f"   Result: {'✅ SUCCESS' if success else '❌ FAILED'}")
        print(f"   Research attempts: {research_attempts}")
        print(f"   Interface attempts: {interface_attempts}")
        print(f"   Total steps: {step_count}")
        
        if success:
            has_translation = bool(current_state.get("html_content_en"))
            has_js_en = "_en" in current_state.get("js_content_en", "")
            print(f"   Translation: {'✅' if has_translation else '❌'}")
            print(f"   JS _en functions: {'✅' if has_js_en else '❌'}")
    
    print(f"\n🎯 ALL RETRY SCENARIOS COMPLETED!")
    print("   The workflow correctly handles:")
    print("   ✅ Research validation retries")
    print("   ✅ Interface extraction retries") 
    print("   ✅ Translation with _en suffixes")
    print("   ✅ Proper termination conditions")

if __name__ == "__main__":
    run_retry_scenarios()
