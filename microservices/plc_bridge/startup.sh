#!/bin/bash
clear

echo "Running startup script for plc-bridge..."

docker build -t plc-bridge-image:latest .
docker run -d --name plc-bridge-container --network host --restart unless-stopped plc-bridge-image:latest

sleep 5

docker logs plc-bridge-container
