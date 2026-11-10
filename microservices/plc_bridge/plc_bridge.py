from pycomm3 import LogixDriver
from threading import Thread, Event
from queue import Queue
from time import sleep
import os

stop_event = Event()

# Producer: reads tags from the controller plc and puts them onto a buffer
def controller_tag_reader(buffer, controller_ip):
    print("[PRODUCER] Thread starting...")
    try:
        with LogixDriver(controller_ip) as plc:
            print(f"[PRODUCER] Connected to the Controller PLC at IP:{controller_ip}")
            while not(stop_event.is_set()):
                # 1. Read required tags from the Controller PLC
                tag_values = {
                    'START': plc.read('Local:1:I.Data.8').value, # Start push button
                    'STOP': not(plc.read('Local:1:I.Data.9').value), # Stop push button
                    'GREEN': plc.read('Local:1:O.Data.0').value, # Green Light - nominal temperature of SenseHAT IMU
                    'ORANGE': plc.read('Local:1:O.Data.5').value, # Orange Light - IMU has overheated
                }
                print("[PRODUCER] Tags acquired from Controller PLC")

                # 2. Place the tags into the shared buffer
                buffer.put(tag_values)
                print(f"[PRODUCER] Tags put into the buffer: {tag_values}")
                
                # 3. Log the collected tags
                
                
                # Sleep for a moment, as to not overdue
                sleep(2)
            print("[PRODUCER] Thread stopped gracefully!")
    except Exception as e:
        print(f"[PRODUCER] PLC Connection error: {e}")

# Consumer: reads tags from the buffer and prints them to the screen
def robot_tag_writer(buffer, robot_ip):
    print("[CONSUMER] Thread starting...")
    try:
        with LogixDriver(robot_ip) as plc:
            print(f"[CONSUMER] Connected to the Robot Arm PLC at IP:{robot_ip}")
            while not(stop_event.is_set()):
                # 1. Read the current tags array from the front of the queue
                tag_values = buffer.get()
        
                # 2. Check to see if the received item is empty
                if tag_values == None:
                    break
        
                # 3. Write the tags to the robot arm plc
                plc.write(
                        ('PLC_IN_6', tag_values['START']),       # Starts/resets robot
                        ('PLC_IN_7', tag_values['STOP']),        # Stops the robot after its currrent cycle
                        ('PLC_IN_11', tag_values['GREEN']),     # Lets the robot know it can go 100% speed
                        ('PLC_IN_12', tag_values['ORANGE']),    # Tells the robot to halt until temperature goes down
                )

                print(f"[CONSUMER] Tag read from the buffer: {tag_values}")
            print("[CONSUMER] Thread stopped gracefully!")
    except Exception as e:
        print(f"[CONSUMER] PLC Connection error: {e}")

# Main entrypoint: pull environment variables and establish consumer-producer threads and shared memory
def main():
    print("[MAIN] Starting bridge program...")
    # Get environment variables
    controller_ip = os.getenv('CONTROLLER_PLC_IP', '192.168.6.32')
    robot_ip = os.getenv('ROBOT_PLC_IP', '192.168.6.41')
    print(f"[MAIN] Retrieved PLC IP addresses: Controller->{controller_ip} Robot->{robot_ip}")

    # Instantiate a shared memory buffer
    buffer = Queue()
    print("[MAIN] Buffer initialized!")
    
    # 1. Start the robot plc tag writer (consumer)
    print("[MAIN] Starting Robot PLC Writer thread...")
    consumer_thread = Thread(target=robot_tag_writer, args=(buffer, robot_ip, ), daemon=True)
    consumer_thread.start()

    # 2. Start controller plc tag reader (producer)
    print("[MAIN] Starting Controller PLC Reader thread...")
    producer_thread = Thread(target=controller_tag_reader, args=(buffer, controller_ip, ), daemon=True)
    producer_thread.start()

    # 3. Wait for both threads to finish
    try:
        while consumer_thread.is_alive() and producer_thread.is_alive():
            consumer_thread.join(timeout=0.1)
            producer_thread.join(timeout=0.1)
    except KeyboardInterrupt:
        print("[MAIN] KeyboardInterruption detected; shutting down...")

if __name__ == "__main__":
    main()
