.PHONY: help start stop backend frontend agent logs clean setup-ai demo

help:
	@echo "ML Monitoring Dashboard Commands:"
	@echo "  start     - Start all services"
	@echo "  stop      - Stop all services"
	@echo "  backend   - Run Rust backend"
	@echo "  frontend  - Run React frontend"
	@echo "  agent     - Run monitoring agent"
	@echo "  logs      - Generate test logs"
	@echo "  setup-ai  - Install Ollama + Gemma for log humanization"
	@echo "  demo      - Start complete demo with all services"
	@echo "  clean     - Clean up containers and volumes"

start:
	docker-compose up -d
	@echo "Infrastructure started. Waiting for services..."
	@sleep 10
	@echo "Services ready!"

stop:
	docker-compose down

backend:
	cd backend && cargo run

frontend:
	cd frontend && npm install && npm start

agent:
	cd agent && cargo run

logs:
	pip3 install kafka-python
	python3 scripts/log_generator.py

setup-ai:
	./scripts/setup_ai.sh

demo:
	./scripts/demo.sh

clean:
	docker-compose down -v
	docker system prune -f
