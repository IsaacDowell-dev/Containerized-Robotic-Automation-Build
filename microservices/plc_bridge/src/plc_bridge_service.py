import time
from dotenv import load_dotenv
import os
import sys
import socket
from pycomm3 import LogixDriver, CommError

# Client logging library
from loggerclient import LoggerClient, LoggerConfig

# Establish the logger config using environment vars
LOGGER_CONFIG = LoggerConfig(
    service_name= "plc-bridge",
    log_ingest_url= os.getenv("LOGGING_INGEST_URL"),
    host_ip= os.getenv("HOST_IP"),
    # Exclude the batch size, flush interval, and max queue size for default values (100, 5.0, 10000)
)

# Load environment variables
load_dotenv()

# Configurations
CONTROLLER_PLC = os.getenv("CONTROLLER_PLC")
ROBOT_PLC = os.getenv("ROBOT_PLC")
POLL_INTERVALL = os.getenv("POLL_INTERVAL", 0.5)
KEEPALIVE_INTERVAL = os.getenv("KEEPALIVE_INTERVAL", 30)
CONNECTION_TIMEOUT = os.getenv("CONNECTION_TIMEOUT", 10)

# Tag mappings (source_tag_on_controller, target_tag_on_robot)
TAG_MAP = {
    ("Local:1:I.Data.8", "PLC_IN_6"),   # START BUTTON
    ("Local:1:I.Data.9", "PLC_IN_7"),   # STOP BUTTON
    ("Local:1:O.Data.0", "PLC_IN_11"),  # GREEN LIGHT
    ("Local:1:O.Data.1", "PLC_IN_12"),  # ORANGE LIGHT
}

""" PLC BRIDGE CLASS """
class PlcBridge:
    def __init__(self, controller: str, robot: str, logger: LoggerClient):
        # Init the logix driver connections to both plcs
        self.controller = LogixDriver(controller, init_tags=True)
        self.robot = LogixDriver(robot, init_tags=True)
        # Keep track of the last activity from either plc
        self._last_activity = 0.0
        self.logger = logger

    def connect(self):
        """ Open both connections, retry with init_tags=False if need to reconnect 
        if a connection isn't made before the timeout window closes, alert log and
        close the program """
        deadline = time.time() + CONNECTION_TIMEOUT

        for plc, name in [(self.controller, "CONTROLLER PLC"), (self.robot, "ROBOT PLC")]:
            if plc.connected:
                # Log a successful connection
                self.logger.info(f"Successfully connected to {name}.")
                continue
            while time.time() < deadline:
                try:
                    # Try to reconnect; skip tag upload
                    plc.init_tags = False
                    plc.open()
                    # Log a successful connection
                    self.logger.info(f"Successfully connected to {name}.")
                    break
                except CommError as e:
                    remaining = deadline - time.time()
                    # Log a warning about how the program isn't reaching the PLC(s)
                    self.logger.warning(f"Failed to connect to the {name}, trying again. Remaining time for connection: {remaining:.2f}")
                    time.sleep(min(1.0, remaining))
            else:
                # Deadline has exceeded
                self.logger.critical(f"Unable to connect to the {name}; shutting down PLC bridge.")
                return False

        return True # Successfully connected

    def _keepalive(self):
        """ Ping both PLCs with a lightweight read to prevent timing out """
        now = time.time()
        if now - self._last_activity > KEEPALIVE_INTERVAL:
            self.controller.read("Controller.Instance")
            self.robot.read("Controller.Instance")
            self._last_activity = now
        
    def bridge_cycle(self):
        """ Singular read from controller -> write to robot cycle """
        self._keepalive()

        # 1. Read tag from the controller plc
        source_tags = [t[0] for t in TAG_MAP] # array of allfirst tags in the TAG_MAP array
        results = self.controller.read(*source_tags)
        if not isinstance(results, list):
            # If only one tag was read, create a single tag array
            results = [results]

        # TODO: Implement a check the tags to see if they are different from standard
        #   If so, log the interaction

        # 2. Create the write list for the robot plc
        writes = []
        for result in results:
            if result and result.error is None:
                target_tag = TAG_MAP[source_tags.index(result.tag)][1] # Get the matching robot plc tag from the TAG_MAP
                writes.append((target_tag, result.value)) # Write the value of the source tags to their respective targets
            else:
                # Log a warning about failing to read tags from the controller 
                self.logger.warning("Unable to read tags from the CONTROLLER PLC.")
                pass

        if writes:
            self.robot.write(*writes)
            self._last_activity = time.time()

    def run(self):
        """ Main loop with reconnection """
        # Log the start of the main control loop
        self.logger.info("Starting main control loop.")
        self.connect()
        while True:
            try:
                self.bridge_cycle()
                time.sleep(POLL_INTERVAL)
            except CommError as e:
                # Log the connection error and try to reconnect
                self.logger.error(f"An error occurred when trying to connect to one of the PLCs. {e}")
                self._reconnect()
            except Exception:
                # Log the unexpected error that occurred
                self.logger.error("An unexpected error occurred, pausing main control loop for 1 second.")
                time.sleep(1)

    def _reconnect(self):
        """ Close and reopen both connections """
        for plc, name in [(self.controller, "CONTROLLER"), (self.robot, "ROBOT")]:
            try:
                plc.close()
            except Exception:
                pass
        
        time.sleep(2) # Brief pause before trying to reconnect
        
        try :
            self.connect()
        except CommError:
            time.sleep(5) # take a longer break before trying again
        
    def close(self):
        self.controller.close()
        self.robot.close()

if __name__ == "__main__":
    # Init the logger
    logger = LoggerClient(LOGGER_CONFIG)

    # Establish the bridge and try run
    bridge = PlcBridge(CONTROLLER_PLC, ROBOT_PLC, logger)

    if not bridge.connect():
        sys.exit(1)

    try:
        bridge.run()
    except KeyboardInterrupt:
        bridge.close()