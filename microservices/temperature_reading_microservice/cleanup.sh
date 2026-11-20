#!/bin/bash

echo "Cleaning up imu-reader-container..."

docker stop imu-reader-container
docker rm imu-reader-container