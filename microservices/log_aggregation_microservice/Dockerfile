FROM python:3.14

WORKDIR /app

COPY dependencies.txt .

RUN pip install --no-cache-dir --upgrade -r dependencies.txt

COPY log_ingestion_api.py .

CMD ["fastapi", "run", "log_ingestion_api.py", "--port", "5000"]
