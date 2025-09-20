#!/usr/bin/env python3

import requests
import json

# Test log messages
test_logs = [
    "ERROR: Connection refused to database server at localhost:5432",
    "FATAL: Out of memory error - cannot allocate 2048MB",
    "ERROR: Disk full - cannot write to /var/log/app.log",
    "WARN: High CPU usage detected - 95% utilization",
    "ERROR: HTTP 500 - Internal server error in user authentication"
]

def test_humanizer():
    print("🤖 Testing AI Log Humanization...")
    
    for i, log_msg in enumerate(test_logs, 1):
        print(f"\n--- Test {i} ---")
        print(f"Original: {log_msg}")
        
        try:
            response = requests.post(
                "http://localhost:8080/api/humanize-log",
                json={"log_message": log_msg},
                timeout=10
            )
            
            if response.status_code == 200:
                result = response.json()
                humanized = result["data"]
                
                print(f"🔍 Explanation: {humanized['human_explanation']}")
                print(f"⚠️  Severity: {humanized['severity']}")
                print(f"🔧 Possible Causes:")
                for cause in humanized['possible_causes']:
                    print(f"   • {cause}")
                print(f"💡 Suggested Fixes:")
                for fix in humanized['suggested_fixes']:
                    print(f"   • {fix}")
                print(f"📊 Confidence: {humanized['confidence']:.1%}")
            else:
                print(f"❌ Error: {response.status_code}")
                
        except Exception as e:
            print(f"❌ Request failed: {e}")

if __name__ == "__main__":
    test_humanizer()
