from pycomm3 import LogixDriver
from sense_hat import SenseHat
from datetime import datetime
import time
import requests
from custom_types import TemperatureLog

# Environment variables
PLC_IP = '192.168.6.32'
ENDPOINT = 'https://192.168.5.23:5000/temperature_logger'

sense = SenseHat()

# Temperature thresholds (C)
NOMINAL_TEMP = 45
WARM_TEMP = 50
CRITICAL_TEMP = 55

# Evaluates the current temperature reading
def eval_temp_state(temp_c):
    if (temp_c <= NOMINAL_TEMP):
        return "NOMINAL"
    elif (NOMINAL_TEMP < temp_c <= WARM_TEMP):
        return "HOT"
    else:
        return "CRITICAL"

# Main control loop
def run():
    while True:
        # Get timestamp
        timestamp = datetime.now().strftime('%Y-%m-%d %H:%M:%S') 

        # Read temperature from SenseHAT IMU
        temp_c = sense.get_temperature()
        
        # Convert temperature to Fahrenheit (logging)
        temp_f = (temp_c * 9/5) + 32
        
        # Evaluate current temperature state
        status = eval_temp_state(temp_c)

        # Batch write boolean states to PLC communication tags
        # plc.write('SenseHAT_Temp', temp_c)
                
        # Format the JSON request object
        log_post_object = TemperatureLog(
            time_of_record = timestamp,
            temp_c = temp_c,
            temp_f = temp_f,
            temperature_status = status
        )

        print(log_post_object)

        # Post the log_post_object to the listening endpoint on the SHPi
        #r = requests.post(ENDPOINT, data=log_post_object)
        #print(r.text)

        time.sleep(1)

# Starting point of the program
def main():
    try:
        #with LogixDriver(PLC_IP) as plc:
            #print(f"Connected to PLC at {PLC_IP}")
        run()
    except Exception as err:
        print(err)
    except KeyboardInterrupt:
        print("Closing temperature reading program...")

if __name__ == "__main__":
    main()
