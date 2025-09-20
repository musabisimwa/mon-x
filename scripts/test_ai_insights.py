#!/usr/bin/env python3

import requests
import json
import time

def test_ai_insights():
    print("🤖 Testing AI Insights System")
    
    # Test the AI insights API
    try:
        response = requests.get("http://localhost:8080/api/ai-insights?agent_id=demo-app-001", timeout=10)
        if response.status_code == 200:
            insights = response.json()
            print("✅ AI Insights Response:")
            print(json.dumps(insights, indent=2))
            
            if insights.get('success') and insights.get('data'):
                data = insights['data']
                print(f"\n🔍 Analysis: {data.get('analysis', 'N/A')}")
                print(f"⚠️  Severity: {data.get('severity', 'N/A')}")
                print(f"🎯 Root Cause: {data.get('root_cause', 'N/A')}")
                print(f"🔧 Suggested Fixes:")
                for fix in data.get('suggested_fixes', []):
                    print(f"   • {fix}")
                print(f"📊 Confidence: {data.get('confidence', 0):.1%}")
        else:
            print(f"❌ API Error: {response.status_code}")
            print(response.text)
    except Exception as e:
        print(f"❌ Request failed: {e}")

    # Check current anomalies
    try:
        response = requests.get("http://localhost:8080/api/anomalies")
        if response.status_code == 200:
            anomalies = response.json()
            print(f"\n📋 Current Anomalies: {len(anomalies.get('data', []))}")
            for i, anomaly in enumerate(anomalies.get('data', [])[:3]):
                print(f"   {i+1}. {anomaly.get('reason', 'Unknown')} (Score: {anomaly.get('score', 0):.2f})")
                if anomaly.get('humanized'):
                    h = anomaly['humanized']
                    print(f"      AI: {h.get('human_explanation', 'N/A')[:100]}...")
    except Exception as e:
        print(f"❌ Anomalies check failed: {e}")

if __name__ == "__main__":
    test_ai_insights()
