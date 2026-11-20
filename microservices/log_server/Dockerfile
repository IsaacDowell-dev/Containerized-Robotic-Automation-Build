FROM python:3.12-slim

ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1

WORKDIR /code

# Install system dependencies
RUN apt-get update && \
	apt-get install -y --no-install-recommends gcc libpq-dev && \
	rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Install python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy the app code into the image
COPY ./app /code/app

# Set ownership
RUN chown -R appuser:appuser /code

USER appuser

EXPOSE 8000

ENV DB_USER="Sandbox2"
ENV DB_PASS="Data_Base_Rocks_101"
ENV DB_HOST="192.168.5.254"
ENV DB_PORT="3306"
ENV DB_NAME="Sandbox2"

# Run with uvicorn
CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8000"] 

# Build with:	$docker build -t log-server-image .
# Run with: 	$docker run -d --name log-server --host 0.0.0.0 --port 8000:8000 log-server-image