from pycomm3 import LogixDriver
from sense_hat import SenseHat
from datetime import datetime
import time
import requests
from custom_types import TemperatureLog, TemperatureThreshold

# Environment variables
PLC_IP = '192.168.6.32'
URL = 'https://192.168.5.23:5000/temperature_logger'

# API for reading data from the SenseHAT IMU on the SenseHAT RPi
sense = SenseHat()

# Temperature thresholds (C)dd
NOMINAL_TEMP = 45
WARM_TEMP = 50

# Evaluates the current temperature reading
def eval_temp_state(temp_c):
    if (temp_c < NOMINAL_TEMP):
        return TemperatureThreshold.NOMINAL
    elif (NOMINAL_TEMP <= temp_c <= WARM_TEMP):
        return TemperatureThreshold.HOT
    else:
        return TemperatureThreshold.CRITICAL

"""
MAIN CONTROL LOOP: read the temperature from the IMU, create a log,
    and send the temperature data to the Controller PLC
"""
def run(plc):
    while True:
        # 1. Get timestamp
        timestamp = datetime.now().strftime('%Y-%m-%d %H:%M:%S') 

        # 2. Read temperature from SenseHAT IMU
        temp_c = sense.get_temperature()
        
        # 3. Convert temperature to Fahrenheit (logging)
        temp_f = (temp_c * 9/5) + 32
        
        # 4. Evaluate current temperature state
        status = eval_temp_state(temp_c)

        # 5. Write the current temperature as an integer to the Controller PLC
        plc.write('SenseHAT_Temp', temp_c)
                
        # 6. Format the JSON request object
        log_post_object = TemperatureLog(
            timestamp = timestamp,
            temperature_c = temp_c,
            temperature_f = temp_f,
            temperature_status = status
        )
        print(log_post_object)

        # 7. Post the log_post_object to the log writer microservice on the same host device
        post_logs(log_post_object)

        time.sleep(1)

"""
LOGGING PROCESS: posts a generated log object to the logging api
"""
def post_logs(post_object):
    try:
        # Send the post request and await a response
        r = requests.post(URL, data=post_object, timeout=1, verify=True)
        r.raise_for_status()
        print(r.text)
    except requests.exceptions.HTTPError as errh:
        print(f"HTTP Error: \n{errh.args[0]}")
    except requests.exceptions.ReadTimeout as errrt:
        print("Timeout")
    except requests.exceptions.ConnectionError as conerr:
        print("Connection error")
    except requests.exceptions.RequestException as errex:
        print(f"Request exception")
    
# Starting point of the program
def main():
    try:
        with LogicDriver(PLC_IP) as plc:
            run(plc)
    except Exception as err:
        print(err)
    except KeyboardInterrupt:
        print("Closing temperature reading program...")

if __name__ == "__main__":
    main()
