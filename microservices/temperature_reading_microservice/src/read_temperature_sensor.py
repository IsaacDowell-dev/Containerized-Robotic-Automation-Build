from pycomm3 import LogixDriver, CommError
from sense_hat import SenseHat
from datetime import datetime
import time
import os
from dotenv import load_dotenv
import requests

from loggerclient import LoggerClient, LoggerConfig

""" Environment variables """
load_dotenv()
PLC_IP = os.getenv("CONTROLLER_PLC")
LOGGER_CONFIG = LoggerConfig(
    service_name = "imu-temperature-reader",
    log_ingest_url = os.getenv("LOGGING_INGEST_URL"),
    host_ip = os.getenv("HOST_IP"),
    # Dont update default batch size, flush interval, and max queue size values (100, 5.0 10000)
)

logger = LoggerClient(LOGGER_CONFIG)

""" API for reading data from the SenseHAT IMU on the SenseHAT RPi """
sense = SenseHat()

""" Constant Temperature thresholds (C) """
NOMINAL_TEMP = 45
WARM_TEMP = 50

""" Variable containing the previous state of the temperaure """
previous_temp_state: str = "NOMINAL"

# Evaluates the current temperature reading
def eval_temp_state(temp_c):
    if (temp_c < NOMINAL_TEMP):
        return "NOMINAL"
    elif (NOMINAL_TEMP <= temp_c <= WARM_TEMP):
        return "HOT"
    else:
        return "CRITICAL"

"""
MAIN CONTROL LOOP: read the temperature from the IMU, create a log,
    and send the temperature data to the Controller PLC
"""
def run(plc):
    logger.info("Successfully connected to the plc; starting to read temperatures from the IMU.")
    while True:
        # Read temperature from SenseHAT IMU
        temp_c = sense.get_temperature()
        
        # Convert temperature to Fahrenheit (logging)
        temp_f = (temp_c * 9/5) + 32
        
        # Evaluate current temperature state
        status = eval_temp_state(temp_c)

        # Write the current temperature as an integer to the Controller PLC
        plc.write('SenseHAT_Temperature_Value', temp_c)

        # Evaluate information and log it to the database
        post_logs(temp_c, temp_f, status)

        time.sleep(1)

"""
LOGGING PROCESS: posts a log object to the logging api
"""
def post_logs(temp_c, temp_f, status):
    # Evaluate if the current temperature state is 
    if status != prev_temp_state:
        # Log and update the new temp state
        prev_temp_state = status
        if status == "NOMINAL":
            logger.info(f"Current temperature state is {status} at a temperature of {temp_f} F ({temp_c} C).")
        elif status == "HOT":
            logger.warning(f"Current temperature state is {status} at a temperature of {temp_f} F ({temp_c} C).")
        else:
            logger.info(f"Current temperature state is {status} at a temperature of {temp_f} F ({temp_c} C), system entering locked state.")
    
# Starting point of the program
def main():
    try:
        with LogixDriver(PLC_IP) as plc:
            run(plc)
    except CommError:
        logger.critical("Unable to connect to the PLC, shutting down.")
    except Exception as err:
        logger.error("An unexpected error occurred, shutting down.")
    except KeyboardInterrupt:
        logger.info("Experienced keyboard interrupt, shutting down.")

if __name__ == "__main__":
    main()
