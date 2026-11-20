FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .

RUN pip install --no-cache-dir -r requirements.txt

COPY plc_bridge_service.py .

ENV CONTROLLER_PLC = "192.168.6.32"
ENV ROBOT_PLC = "192.168.6.41"

CMD ["python", "plc_bridge_service.py"]

# build with    $ docker build -t plcbridgeimage .
# run with      $docker run --network host --restart unless-stopped plc-bridge