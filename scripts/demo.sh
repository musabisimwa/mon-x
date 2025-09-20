#!/bin/bash

echo "🚀 Starting Mon-X ML Monitoring Dashboard Demo..."

# Check if services are running
echo "📋 Checking services..."

# Start infrastructure if not running
if ! docker ps | grep -q kafka; then
    echo "🐳 Starting infrastructure..."
    make start
    sleep 10
fi

# Start backend in background
echo "🦀 Starting Rust backend..."
cd backend && cargo run &
BACKEND_PID=$!
sleep 5

# Start frontend in background  
echo "⚛️  Starting React frontend..."
cd ../frontend && npm start &
FRONTEND_PID=$!
sleep 5

# Start agent in background
echo "🤖 Starting monitoring agent..."
cd ../agent && cargo run &
AGENT_PID=$!
sleep 3

echo ""
echo "✅ All services started!"
echo ""
echo "🌐 Dashboard: http://localhost:3000"
echo "🔧 Backend API: http://localhost:8080"
echo ""
echo "📊 Features available:"
echo "  • Real-time system metrics (CPU, Memory, Disk, Network, GPU)"
echo "  • AI-powered log humanization with Gemma 2B"
echo "  • Clickable app widgets with health status"
echo "  • Detailed app dashboards with graphs"
echo "  • ML anomaly detection (3 algorithms)"
echo "  • HTTP calls & database query monitoring"
echo ""
echo "🧪 Test AI log analysis:"
echo "  python3 scripts/test_humanizer.py"
echo ""
echo "Press Ctrl+C to stop all services..."

# Wait for interrupt
trap 'echo "🛑 Stopping services..."; kill $BACKEND_PID $FRONTEND_PID $AGENT_PID 2>/dev/null; exit' INT
wait
