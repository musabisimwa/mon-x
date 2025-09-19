#!/usr/bin/env python3
import json
import time
import random
from kafka import KafkaProducer
from datetime import datetime

producer = KafkaProducer(
    bootstrap_servers=['localhost:9092'],
    value_serializer=lambda x: json.dumps(x).encode('utf-8')
)

services = ['auth-service', 'user-service', 'payment-service', 'notification-service']
levels = ['INFO', 'WARN', 'ERROR', 'DEBUG']
messages = [
    'Request processed successfully',
    'Database connection established',
    'Cache miss for key',
    'Authentication failed',
    'Payment processing error',
    'User session expired',
    'Rate limit exceeded',
    'Service unavailable'
]

def generate_log():
    return {
        'timestamp': datetime.utcnow().isoformat() + 'Z',
        'level': random.choices(levels, weights=[70, 20, 5, 5])[0],
        'message': random.choice(messages),
        'service': random.choice(services),
        'trace_id': f"trace-{random.randint(100000, 999999)}" if random.random() > 0.3 else None
    }

def main():
    print("Starting log generator...")
    try:
        while True:
            log = generate_log()
            producer.send('logs', value=log)
            
            # Simulate burst of errors occasionally
            if random.random() < 0.05:
                for _ in range(random.randint(5, 15)):
                    error_log = generate_log()
                    error_log['level'] = 'ERROR'
                    error_log['message'] = 'Critical system error detected'
                    producer.send('logs', value=error_log)
                    time.sleep(0.1)
            
            time.sleep(random.uniform(0.1, 0.5))
            
    except KeyboardInterrupt:
        print("Stopping log generator...")
    finally:
        producer.close()

if __name__ == '__main__':
    main()
