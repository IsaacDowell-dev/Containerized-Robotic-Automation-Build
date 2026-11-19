FROM python:3.9-slim

WORKDIR /app

# Copy file containing required modules
COPY dependencies.txt .

# Install all modules in the dependencies.txt file
RUN pip install --no-cache-dir -r dependencies.txt

# Copy the application code
COPY plc_bridge.py .

# Run the application
CMD ["python", "plc_bridge.py"]
