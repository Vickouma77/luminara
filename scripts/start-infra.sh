#!/bin/bash

# Start all infrastructure services
echo "Starting infrastructure services..."

docker-compose up -d postgres redis kafka zookeeper jaeger prometheus grafana

echo "Waiting for services to be ready..."
sleep 10

echo "Infrastructure services started!"
echo "PostgreSQL: localhost:5432"
echo "Redis: localhost:6379"
echo "Kafka: localhost:9092"
echo "Jaeger UI: http://localhost:16686"
echo "Prometheus: http://localhost:9090"
echo "Grafana: http://localhost:3000"
