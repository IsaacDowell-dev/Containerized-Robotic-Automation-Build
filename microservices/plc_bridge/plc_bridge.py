from pycomm3 import LogixDriver
import sys
import time

"""
ENTRY POINT: establishes connections to both PLCs and starts the main control loop
"""
def main():
    try:
        # System arguments: plc IP addresses
        CONTROL_PLC_IP = sys.argv[1] # if len(sys.argv) > 1 else '192.168.6.32' 
        # ROBOT_PLC_IP = sys.argv[2] # if len(sys.argv> > 2 else '192.168.6.41'
        
        # 1. Connect to the controller PLC
        print(f"Connecting to Controller PLC: {CONTROL_PLC_IP}...")
        with LogixDriver(CONTROL_PLC_IP) as controller:
            get_controller_tags(controller)
    except Exception as e:
        print(f"Communication fault: {e}")
    except KeyboardInterrupt:
        print("Closing PLC bridge application...")

"""
MAIN CONTROL LOOP: read tags from both PLCs and act accordingly
"""
def get_controller_tags(controller_plc_ip):
    while True:
        # 1. Read tags from the controller plc
        start_btn_value = controller_conn.read('Local:1:I.Data.8').value
        stop_btn_value = controller_conn.read('Local:1:I.Data.9').value
        green_light_value = controller_conn.read('Local:1:O.Data.0').value
        yellow_light_value = controller_conn.read('Local:1:O.Data.1').value
        red_light_value = controller_conn.read('Local:1:O.Data.2').value
        orange_light_value = controller_conn.read('Local:1:O.Data.5').value
        
        # 2. Determine states from controller
        start_button_pressed = start_btn_value
        stop_button_pressed = not(stop_btn_value)
        
        temperature_state = ""
        if (green_light_value):
            temperature_state = "NOMINAL"
        elif (yellow_light_value):
            temperature_state = "HOT"
        elif (red_light_value):
            temperature_state = "CRITICAL"
        else:
            temperature_state = "NONE"
        locked = orange_light_value

        # 2. Log Controller tags
        print(f"Start Button Pressed: {start_button_pressed} | Stop Button Pressed: {stop_button_pressed} | Current Temperature State: {temperature_state} | Is System Locked: {locked}")

        
        
        time.sleep(1)

if __name__ == "__main__":
    main()
