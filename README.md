# Mon-X

**ML-Enhanced Monitoring Dashboard with Real-time Log Processing and Agent-based Architecture**

A production-ready monitoring solution that ingests, processes, and analyzes logs in real-time using machine learning for anomaly detection and a React frontend for interactive visualization.

## 🚀 Features

- **Real-time Log Processing**: Kafka-based streaming at 50k+ events/s
- **ML Anomaly Detection**: 3 complementary algorithms (Statistical, Random Cut Forest, Log Embedding)
- **Agent-based Monitoring**: Lightweight sidecars for distributed system monitoring
- **Interactive Dashboard**: React + Material-UI with real-time WebSocket updates
- **Scalable Architecture**: Kubernetes-ready with Docker containerization

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────┐    ┌─────────────────┐
│   Log Sources   │───▶│    Kafka     │───▶│  Rust Backend   │
└─────────────────┘    └──────────────┘    └─────────────────┘
                                                     │
┌─────────────────┐    ┌──────────────┐             │
│ React Frontend  │◀───│  WebSocket   │◀────────────┘
└─────────────────┘    └──────────────┘             │
                                                     ▼
┌─────────────────┐    ┌──────────────┐    ┌─────────────────┐
│   OpenSearch    │◀───│  ML Service  │◀───│   Agent Data    │
│   Dashboards    │    │   (3 Algos)  │    │   Collection    │
└─────────────────┘    └──────────────┘    └─────────────────┘
                                                     ▲
┌─────────────────┐    ┌──────────────┐             │
│ Monitoring Agent│───▶│  HTTP API    │─────────────┘
│   (Sidecar)     │    │   /agents    │
└─────────────────┘    └──────────────┘
```

## 🛠️ Tech Stack

- **Backend**: Rust (Actix-web, SmartCore ML)
- **Frontend**: React, Material-UI, Recharts
- **Streaming**: Apache Kafka
- **Storage**: OpenSearch
- **Agent**: Rust (System metrics, log collection)
- **Deployment**: Docker, Kubernetes-ready

## 🚀 Quick Start

```bash
# Start infrastructure
make start

# Run components (in separate terminals)
make backend   # Rust API server
make frontend  # React dashboard  
make agent     # Monitoring agent
make logs      # Generate test data
```

## 📊 ML Algorithms

1. **Statistical Analysis**: Detects outliers in log message patterns
2. **Random Cut Forest**: Identifies error rate spikes (3x threshold)
3. **Log Embedding**: Finds rare error patterns using tokenization

## 🎯 Performance Targets

- **Throughput**: 50k+ events/second
- **Latency**: <100ms anomaly detection
- **Accuracy**: 95%+ ML model performance
- **Scalability**: Horizontal scaling with agents

## 🔧 Configuration

### Agent Setup
Edit `agent/config.yaml`:
```yaml
agent:
  name: "your-service-name"
  server_url: "http://localhost:8080"
  report_interval: 5

collection:
  metrics: true
  logs: true
  resources: true
```

## 📈 Monitoring

- **Dashboard**: http://localhost:3000
- **API**: http://localhost:8080/api/*
- **OpenSearch**: http://localhost:9200
- **Dashboards**: http://localhost:5601

## 🏢 Enterprise Ready

- **Security**: JWT auth, mTLS, RBAC
- **Observability**: Distributed tracing ready
- **Deployment**: Helm charts, auto-scaling
- **Monitoring**: Health checks, metrics export

---


