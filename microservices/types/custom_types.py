from pydantic import BaseModel

""" Logging object structures """

# Temperature logs from the temp reader service
class TemperatureLog(BaseModel):
    time_of_record: str
    temp_c: float
    temp_f: float
    temperature_status: "NOMINAL" | "HOT" | "CRITICAL"
