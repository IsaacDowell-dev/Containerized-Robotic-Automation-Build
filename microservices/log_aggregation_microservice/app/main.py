from fastapi import FastAPI
from types import TemperatureLog

app = FastAPI()

@app.post("/temperature_logging/")
async def temperature_logger(request: TemperatureLog):
    # Format the incoming temperature log as a DB Log Entry

    # Pass the formatted log to DB Batch Processor

    return({'message': 'Temperature logged successfully'})
