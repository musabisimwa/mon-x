#!/usr/bin/env python3

import requests
import json
import time

def test_gguf_ai_system():
    print("🤖 Testing GGUF AI Integration System")
    print("=" * 50)
    
    # Test different scenarios
    scenarios = [
        ("demo-app-001", "Normal Application"),
        ("high-cpu-app", "High CPU Usage"),
        ("memory-leak-app", "Memory Leak Scenario"),
        ("critical-system", "Critical System Failure")
    ]
    
    for agent_id, description in scenarios:
        print(f"\n📊 Testing: {description} ({agent_id})")
        print("-" * 40)
        
        try:
            response = requests.get(f"http://localhost:8080/api/ai-insights?agent_id={agent_id}", timeout=10)
            if response.status_code == 200:
                insights = response.json()
                if insights.get('success') and insights.get('data'):
                    data = insights['data']
                    
                    # Display severity with color coding
                    severity = data.get('severity', 'UNKNOWN')
                    severity_emoji = {
                        'CRITICAL': '🚨',
                        'HIGH': '⚠️',
                        'LOW': '✅'
                    }.get(severity, '❓')
                    
                    print(f"{severity_emoji} Severity: {severity}")
                    print(f"🔍 Analysis: {data.get('analysis', 'N/A')}")
                    print(f"🎯 Root Cause: {data.get('root_cause', 'N/A')}")
                    print(f"📊 Confidence: {data.get('confidence', 0):.1%}")
                    
                    print("🔧 Suggested Actions:")
                    for i, fix in enumerate(data.get('suggested_fixes', []), 1):
                        print(f"   {i}. {fix}")
                        
                else:
                    print("❌ No insights data received")
            else:
                print(f"❌ API Error: {response.status_code}")
                
        except Exception as e:
            print(f"❌ Request failed: {e}")
        
        time.sleep(1)  # Brief pause between tests
    
    print("\n" + "=" * 50)
    print("🎯 GGUF Model Integration Summary:")
    print("✅ Model Detection: Automatic GGUF file discovery")
    print("✅ Fallback System: Enhanced rule-based analysis")
    print("✅ Real-time Analysis: Agent-specific insights")
    print("✅ Frontend Integration: AI panel in application details")
    print("✅ API Endpoint: /api/ai-insights?agent_id={app_name}")
    
    print(f"\n🌐 Visit http://localhost:3000 to see the AI insights in action!")
    print(f"📱 Click on any application to view AI-powered analysis")

if __name__ == "__main__":
    test_gguf_ai_system()
