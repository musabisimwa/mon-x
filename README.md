# Mon-X Monitoring Dashboard

**Real-time System Monitoring with ML-powered Anomaly Detection**

A comprehensive monitoring solution that collects system metrics, logs, and traces from distributed applications using lightweight agents, processes data in real-time, and provides intelligent insights through machine learning.

## 🚀 Features

- **Real-time Agent Monitoring**: Lightweight agents collect system metrics, logs, and traces
- **ML Anomaly Detection**: Intelligent pattern recognition and outlier detection
- **Interactive Dashboard**: React frontend with real-time updates
- **Multi-source Data Collection**: System metrics, Docker logs, network traces, process monitoring
- **Scalable Architecture**: Microservices-ready with Docker containerization

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────┐    ┌─────────────────┐
│ Monitoring Agent│───▶│ HTTP API     │───▶│  Rust Backend   │
│ (System Metrics)│    │ /agents/*    │    │ (Actix-web)     │
└─────────────────┘    └──────────────┘    └─────────────────┘
                                                     │
┌─────────────────┐    ┌──────────────┐             │
│ React Frontend  │◀───│  REST API    │◀────────────┘
│ (Material-UI)   │    │ /api/*       │             │
└─────────────────┘    └──────────────┘             │
                                                     ▼
┌─────────────────┐    ┌──────────────┐    ┌─────────────────┐
│   OpenSearch    │◀───│  ML Service  │◀───│   Data Store    │
│   (Optional)    │    │ (Anomalies)  │    │ (In-Memory)     │
└─────────────────┘    └──────────────┘    └─────────────────┘
```

## 🛠️ Tech Stack

- **Backend**: Rust (Actix-web, SmartCore ML, Tokio)
- **Frontend**: React 18, Material-UI, Recharts, Axios
- **Agent**: Rust (System monitoring, Docker integration)
- **Storage**: In-memory + OpenSearch (optional)
- **Infrastructure**: Docker Compose, Kafka, Zookeeper

## 🚀 Quick Start

### Prerequisites
- **Rust** (1.70+): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Node.js** (18+): `curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash - && sudo apt-get install -y nodejs`
- **Docker & Docker Compose**: `sudo apt install docker.io docker-compose`

### 1. Start Infrastructure
```bash
cd ml-monitoring-dashboard
docker-compose up -d
```

### 2. Start Backend (Terminal 1)
```bash
cd backend
cargo run
# Backend starts on http://localhost:8080
```

### 3. Start Frontend (Terminal 2)
```bash
cd frontend
npm install
npm start
# Frontend starts on http://localhost:3000
```

### 4. Start Monitoring Agent (Terminal 3)
```bash
cd agent
cargo build --release
./target/release/monitoring-agent
# Agent starts collecting system data
```

### 5. Access Dashboard
Open http://localhost:3000 in your browser to see:
- **Main Dashboard**: Discovered applications and agents
- **Agent Details**: Click on "comprehensive-agent-001" to view detailed metrics and logs

## 📊 What Gets Monitored

### System Metrics
- **CPU**: Per-core usage, load averages
- **Memory**: Used/free memory, swap usage
- **Disk**: I/O statistics, filesystem usage
- **Network**: Interface traffic, listening ports
- **Temperature**: System sensors
- **GPU**: NVIDIA GPU utilization (if available)

### Logs & Traces
- **System Logs**: Real journalctl entries
- **Docker Logs**: Container logs from running services
- **Network Traces**: Active listening sockets
- **Process Data**: Running processes with CPU/memory usage

### Health Monitoring
- **HTTP Endpoints**: Configurable health check URLs
- **Service Discovery**: Automatic application detection
- **Real-time Updates**: 5-second refresh intervals

## 🔧 Configuration

### Agent Configuration (`agent/config.yaml`)
```yaml
agent:
  name: "comprehensive-agent-001"
  server_url: "http://localhost:8080"
  report_interval: 5

collection:
  metrics: true
  logs: true
  traces: true
  processes: true
  health: true
  docker: true
  nginx_log_path: "/var/log/nginx/access.log"
  health_check_urls:
    - "http://localhost:8080/health"
    - "http://localhost:3000/api/health"
```

## 📈 API Endpoints

### Agent Registration & Data
- `POST /api/agents/register` - Register new agent
- `POST /api/agents/metrics` - Submit metrics data
- `POST /api/agents/logs` - Submit log data
- `POST /api/agents/traces` - Submit trace data
- `POST /api/agents/processes` - Submit process data
- `POST /api/agents/health` - Submit health checks

### Data Retrieval
- `GET /api/agents` - List registered agents
- `GET /api/metrics` - Get system metrics
- `GET /api/logs` - Get collected logs
- `GET /api/processes` - Get process information
- `GET /api/anomalies` - Get ML-detected anomalies

## 🎯 Monitoring Targets

- **Dashboard**: http://localhost:3000
- **Backend API**: http://localhost:8080
- **OpenSearch**: http://localhost:9200 (if running)
- **Kafka**: localhost:9092 (if running)

## 🔍 Troubleshooting

### Agent Issues
```bash
# Check agent logs
cd agent && ./target/release/monitoring-agent

# Verify backend connectivity
curl http://localhost:8080/api/agents
```

### Backend Issues
```bash
# Check backend logs
cd backend && cargo run

# Test API endpoints
curl http://localhost:8080/health
```

### Frontend Issues
```bash
# Check frontend logs
cd frontend && npm start

# Verify API proxy
curl http://localhost:3000/api/agents
```

## 🚀 Production Deployment

### Docker Build
```bash
# Build all components
docker-compose build

# Run in production mode
docker-compose -f docker-compose.prod.yml up -d
```

### Kubernetes Deployment
```bash
# Apply manifests
kubectl apply -f k8s/

# Check status
kubectl get pods -l app=mon-x
```

## 📊 ML Features

- **Anomaly Detection**: Statistical analysis of metrics and logs
- **Pattern Recognition**: Identifies unusual system behavior
- **Predictive Alerts**: Early warning system for potential issues
- **Adaptive Thresholds**: Self-tuning alert boundaries

## 🔒 Security

- **Agent Authentication**: Secure agent registration
- **API Rate Limiting**: Protection against abuse
- **Data Validation**: Input sanitization and validation
- **Network Security**: Configurable TLS/SSL support

---

**Built with ❤️ using Rust and React**
