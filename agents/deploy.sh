#!/bin/bash

echo "🚀 Deploying Mon-X Agent to Spring Boot Container"

# Build the agent image
cd java
docker build -t monx-agent:latest .

# Deploy multiple Spring Boot apps with Mon-X agents
echo "📦 Starting Spring Boot applications with Mon-X agents..."

# App 1: E-commerce API
docker run -d --name ecommerce-api \
  -p 8081:8080 \
  -e AGENT_ID=ecommerce-api \
  -e KAFKA_SERVERS=host.docker.internal:9092 \
  --add-host=host.docker.internal:host-gateway \
  monx-agent:latest

# App 2: User Service
docker run -d --name user-service \
  -p 8082:8080 \
  -e AGENT_ID=user-service \
  -e KAFKA_SERVERS=host.docker.internal:9092 \
  --add-host=host.docker.internal:host-gateway \
  monx-agent:latest

# App 3: Payment Gateway
docker run -d --name payment-gateway \
  -p 8083:8080 \
  -e AGENT_ID=payment-gateway \
  -e KAFKA_SERVERS=host.docker.internal:9092 \
  --add-host=host.docker.internal:host-gateway \
  monx-agent:latest

echo "✅ Deployed 3 Spring Boot applications with Mon-X agents"
echo "🌐 Applications:"
echo "   - ecommerce-api: http://localhost:8081/actuator/health"
echo "   - user-service: http://localhost:8082/actuator/health" 
echo "   - payment-gateway: http://localhost:8083/actuator/health"
echo ""
echo "📊 Check Mon-X dashboard: http://localhost:3000"
echo "🤖 AI insights will appear for each application"

# Wait and show status
sleep 10
echo ""
echo "=== Container Status ==="
docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"

echo ""
echo "=== Testing Agent Registration ==="
sleep 5
curl -s http://localhost:8080/api/agents | jq '.data[] | .name'
