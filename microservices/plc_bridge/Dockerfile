FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .

RUN pip install --no-cache-dir -r requirements.txt

COPY src/ .

# Move these to the yaml file
ENV CONTROLLER_PLC = "192.168.6.32"
ENV ROBOT_PLC = "192.168.6.41"
ENV LOGGING_INGEST_URL = "http://192.168.5.23/api/ingest/batch"
ENV HOST_IP = "192.168.5.23"

CMD ["python", "plc_bridge_service.py"]

# build with    $ docker build -t plc-bridge-image .
# run with      $ docker run --network host --restart unless-stopped plc-bridge
# running using the host network is crucial for allowing the container to communicate with the plcs