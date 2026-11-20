#!/bin/bash
clear

echo "Running startup script for imu-reader..."

docker build -t imu-reader-image:latest .
docker run -d --name imu-reader-container --network host --restart unless-stopped imu-reader-image:latest

sleep 5

docker logs imu-reader-container
