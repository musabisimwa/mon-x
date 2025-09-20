#!/bin/bash

echo "Setting up AI log humanization with Ollama + Gemma 2B..."

# Install Ollama if not present
if ! command -v ollama &> /dev/null; then
    echo "Installing Ollama..."
    curl -fsSL https://ollama.ai/install.sh | sh
fi

# Start Ollama service
echo "Starting Ollama service..."
ollama serve &
sleep 5

# Pull Gemma 2B model (lightweight)
echo "Downloading Gemma 2B model..."
ollama pull gemma2:2b

echo "✅ AI setup complete!"
echo "Test with: curl -X POST http://localhost:11434/api/generate -d '{\"model\":\"gemma2:2b\",\"prompt\":\"Hello\"}'"
