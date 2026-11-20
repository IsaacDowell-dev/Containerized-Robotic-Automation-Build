#!/bin/bash

echo "Cleaning up plc-bridge-container..."

docker stop plc-bridge-container
docker rm plc-bridge-container
