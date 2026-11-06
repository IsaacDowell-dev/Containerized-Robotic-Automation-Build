from pydantic import BaseModel
from enum import Enum

"""
SHARED ENUM TYPES
"""
class TemperatureThreshold(Enum):
    NOMINAL = "NOMINAL"
    HOT = "HOT"
    CRITICAL = "CRITICAL"

"""
Logging object structures 
"""
# Temperature logs from the temp reader service
class TemperatureLog(BaseModel):
    timestamp: str
    temperature_c: float
    temperature_f: float
    temperature_status: TemperatureThreshold

# Controller plc logs from the plc bridge service
class ControllerPLCTags(BaseModel):
    timestamp: str
    start_btn_pressed: bool
    stop_btn_pressed: bool
    green_light_on: bool        # Nominal temperature state
    yellow_light_on: bool       # Hot temperature state
    red_light_on: bool          # Critical tmperature state
    orange_light_on: bool       # System is locked

