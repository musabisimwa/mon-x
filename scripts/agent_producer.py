#!/usr/bin/env python3

import json
import time
import random
from kafka import KafkaProducer
import requests

# Agent configuration
AGENT_ID = "demo-app-001"
BACKEND_URL = "http://localhost:8080"

def register_agent():
    """Register agent with backend to create topics"""
    agent_data = {
        "name": AGENT_ID,
        "capabilities": {
            "logs": True,
            "metrics": True,
            "traces": True,
            "processes": True
        }
    }
    
    try:
        response = requests.post(f"{BACKEND_URL}/api/agents/register", json=agent_data)
        print(f"✅ Agent registered: {response.json()}")
    except Exception as e:
        print(f"❌ Registration failed: {e}")

def create_producer():
    """Create Kafka producer"""
    return KafkaProducer(
        bootstrap_servers=['localhost:9092'],
        value_serializer=lambda v: json.dumps(v).encode('utf-8')
    )

def send_logs(producer):
    """Send log events to agent-specific topic"""
    log_levels = ["INFO", "WARN", "ERROR", "DEBUG"]
    messages = [
        "User authentication successful",
        "Database connection established",
        "Cache miss for key: user_123",
        "HTTP request processed in 45ms",
        "Memory usage at 78%",
        "Connection timeout to external API",
        "Invalid input validation failed",
        "Background job completed successfully"
    ]
    
    log_event = {
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ"),
        "level": random.choice(log_levels),
        "message": random.choice(messages),
        "service": "demo-service",
        "agent_id": AGENT_ID,
        "source": "application",
        "trace_id": f"trace-{random.randint(1000, 9999)}"
    }
    
    topic = f"logs-{AGENT_ID}"
    producer.send(topic, log_event)
    print(f"📋 Sent log to {topic}: {log_event['level']} - {log_event['message']}")

def send_metrics(producer):
    """Send metric events to agent-specific topic"""
    metrics = [
        {"type": "cpu", "unit": "percent", "value": random.uniform(10, 95)},
        {"type": "memory", "unit": "percent", "value": random.uniform(30, 85)},
        {"type": "disk", "unit": "percent", "value": random.uniform(20, 70)},
        {"type": "network", "unit": "bytes/sec", "value": random.uniform(1000, 50000)},
        {"type": "gpu", "unit": "percent", "value": random.uniform(0, 100)},
    ]
    
    metric = random.choice(metrics)
    metric_event = {
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ"),
        "agent_id": AGENT_ID,
        "metric_type": metric["type"],
        "value": metric["value"],
        "unit": metric["unit"],
        "labels": {
            "host": "demo-host",
            "environment": "production"
        }
    }
    
    topic = f"metrics-{AGENT_ID}"
    producer.send(topic, metric_event)
    print(f"📊 Sent metric to {topic}: {metric['type']} = {metric['value']:.1f} {metric['unit']}")

def send_traces(producer):
    """Send trace events to agent-specific topic"""
    operations = ["user.login", "db.query", "api.call", "cache.get", "file.read"]
    
    trace_event = {
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ"),
        "agent_id": AGENT_ID,
        "trace_id": f"trace-{random.randint(10000, 99999)}",
        "span_id": f"span-{random.randint(1000, 9999)}",
        "operation": random.choice(operations),
        "duration_ms": random.randint(5, 500),
        "status": random.choice(["success", "error", "timeout"])
    }
    
    topic = f"traces-{AGENT_ID}"
    producer.send(topic, trace_event)
    print(f"🔍 Sent trace to {topic}: {trace_event['operation']} ({trace_event['duration_ms']}ms)")

def main():
    print(f"🚀 Starting agent producer for: {AGENT_ID}")
    
    # Register agent first
    register_agent()
    time.sleep(2)
    
    # Create producer
    producer = create_producer()
    
    print("📡 Sending data to Kafka topics...")
    
    try:
        for i in range(20):
            # Send different types of data
            send_logs(producer)
            
            if i % 3 == 0:
                send_metrics(producer)
            
            if i % 5 == 0:
                send_traces(producer)
            
            time.sleep(2)
            
    except KeyboardInterrupt:
        print("\n🛑 Stopping producer...")
    finally:
        producer.close()
        print("✅ Producer closed")

if __name__ == "__main__":
    main()
