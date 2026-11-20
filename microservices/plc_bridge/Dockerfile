FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .

RUN pip install --no-cache-dir -r requirements.txt

COPY plc_bridge_service.py .

CMD ["python", "plc_bridge_service.py"]