#!/bin/bash

echo "🚀 Building ML Monitoring Dashboard..."

# Build Docker images
echo "📦 Building Docker images..."
docker-compose -f docker-compose.prod.yml build

# Build agent binary
echo "🦀 Building agent binary..."
cd agent && cargo build --release && cd ..

# Create deployment package
echo "📁 Creating deployment package..."
mkdir -p dist/agent
cp agent/target/release/monitoring-agent dist/agent/monx-agent
cp agent/config.yaml dist/agent/
chmod +x dist/agent/monx-agent

echo "✅ Build complete!"
echo "📋 Usage:"
echo "  Dashboard: docker-compose -f docker-compose.prod.yml up"
echo "  Agent: Copy dist/agent/ to your application server"
