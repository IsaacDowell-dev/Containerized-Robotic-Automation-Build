# Containerized-Robotic-Automation-Build

# Purpose
The purpose of CRAB is to automate the deployment of robotic control systems. The software will have containerized systems for sending signals, collecting logs, displaying logs, and reading input. The system will allow for quick deployment of software connecting PLCs and robotic equipment. Additionally, it will support logging through NAS and displaying logs.

# Scope
- HMI Raspberry Pi Role: Visual dashboard for the current state of the PLC, robot arm, and network connections. Also acts as a secondary controller for the robot arm.
- Sense HAT Raspberry Pi Role: Bridge between all other connected nodes (HMIPi, Laptop, Robot Arm, PLC), and physical temperature sensor.
- Remote Storage: In-lab NAS server for uploading program logs from the SHPi.
- Additional Nodes: Robotic Arm and Connected PLC (R2D2), team-member laptop, and station 2 CompactLogix PLC.

# Out-of-Scope Boundary
Due to time limitations, we plan on excluding the implementation of Docker Swarm for automated container deployment to the Raspberry Pis, and will instead opt for a BASH script to automatically SSH into each Pi and start their containers. For similar reasons, we will exclude the Sense HAT IMU temperature sensing as a stop condition for the robot.

# Project Requirements

## Functional Requirements

### Six or More UDP/TCP Sockets
1. The system uses 11 non-web (TCP or UDP) sockets for communication of input signals, logs, and containerized startup. 
2. The teammate laptop starts the containers on the Sense HAT Pi and HMI Pi via Docker TCP sockets. 
3. The Station 2 CompactLogix PLC (server) broadcasts tags to the Robot Arm PLC and Sense HAT Pi (clients) using UDP sockets. 
4. The Robot Arm PLC (server) broadcasts tags to the Station 2 PLC and Sense HAT Pi (clients) using UDP sockets. 
5. The Sense HAT Pi (server) sends CIP tags to the Robot Arm PLC and Station 2 PLC (clients) using UDP sockets. 
6. The Sense HAT Pi (server) sends system state information to the team member laptop and HMI Pi (clients) via TCP sockets. 
7. The Sense HAT Pi (server) listens for PLC instructions from the team member laptop and HMI Pi (clients) via TCP sockets. 
8. The Sense HAT Pi (client) streams logs to the in-classroom NAS (server) using a TCP port.

### Multi-Node Distributed Software Execution
9. The HMI Pi, Sense HAT Pi, and team member’s laptop each run the Docker daemon. 
10. The HMI Pi and team member laptop run containerized GUI dashboard applications (dashboard.rs) that queries state changes from the Sense HAT Pi.
11. The HMI Pi runs a dockerfile that starts up the microservice in the node.
12. The Sense HAT Pi runs a containerized PLC Tag Requester microservice (query_plc_states.py) that queries the Station 2 PLC and the Robotic Arm PLC for current tag values.
13. The Sense HAT Pi runs a containerized PLC Instruction Reader microservice (listen_plc_instructions.py) that listens for POST requests from the laptop and HMI Pi of instructions to execute on the Station 2 PLC.
14. The Sense HAT Pi runs a Network Monitor microservice (net_monitor.sh) that pings each node in the system and ensures that all are connected.
15. The Sense HAT Pi runs a State Formatter microservice (format_state.py) that reads state data from the Tag PLC Tag Requester and Network Monitor microservices and formats them as system state objects.
16. The Sense HAT Pi runs a Log Archiver microservice (archive_logs.sh) that reads state data from the State Formatter microservice and streams it to the in-classroom NAS.
17. The Sense HAT Pi runs a Dashboard Updater microservice (update_state.py) that listens for GET queries from the team member laptop and HMI Pi dashboard apps.
18. The Sense HAT Pi runs a Tag Generator microservice (generate_tags.py) that gets instructions from the PLC Instruction Reader microservice, converts them to CID Tags, and POSTS those tags to the Station 2 PLC.
19. The Sense HAT Pi runs a compose.sh program that starts all Docker microservices.
20. The team member laptop runs a compose.yaml file that starts all microservices in the laptop, HMI Pi, and Sense HAT Pi via a start_nodes.sh script.
21. The Robot Arm runs a program (CRAB) that actuates the arm.
22. The Robot Arm PLC runs a ladder logic program (crab_robot_controller.acd) that controls the arm and tracks the state of the arm and Station 2 PLC communication.
23. The Station 2 PLC runs a ladder logic program (crab_system_controller.acd) that reads input from the control board, update state indicators (LED Lights), and listens for CID tags from the Sense HAT Pi.

### Real-World Physical or System-Level Inputs and Network State Changes
24. The HMiPi will accept input on its touchscreen at a rate of at most 1/sec.
25. The team member laptop will accept mouse inputs  at a rate of at most 1/sec.
26. The Station 2 PLC will accept momentary contact button presses at a rate of at most 1/sec.
27. Inputs from the Station 2 PLC, team member laptop, and HMI Pi will cause shining LED lights on the Station 2 PLC.
28. Inputs from the Station 2 PLC, team member laptop ,and HMI Pi will cause actuation of the Robot Arm.

### HMI Pi Touchscreen Live User Interface
29. The HMiPi will run a GUI which will display real-time logs from the system and the current positional state of the robotic arm.
30. The team member laptop will run a GUI that displays real-time logs from the system and current positional state of the robotic arm.

### Docker Containerization & Deployment 
31. The Sense HAT Pi PLC Tag Requester microservice listens for UDP dataframes on port 44818.
32. The Sense HAT Pi PLC Tag Requester microservice posts UDP dataframes on port 8010.
33. The Sense HAT Pi PLC Instruction Reader microservice listens for UDP dataframes on port 8001.
34. The Sense HAT Pi PLC Instruction Reader microservice posts TCP packets on port 8011.
35. The Sense HAT Pi Network Monitor microservice posts TCP packets on port 8010.
36. The Sense HAT Pi State Formatter microservice listens for UPD dataframes on port 8010 and 8012.
37. The Sense HAT Pi State Formatter microservice posts TCP packets on port 8020.
38. The Sense HAT Pi Log Archiver microservice listens for UDP dataframes on port 8020.
39. The Sense HAT Pi Log Archiver microservice posts TCP packets on port 8030.
40. The Sense HAT Pi Dashboard Updater microservice listens for UDP dataframes on port 8020.
41. The Sense HAT Pi Dashboard Updater microservice listens for TCP requests on port 8031.
42. The Sense HAT Pi Tag Generator microservice listens for TCP packets on port 8011.
43. The Sense HAT Pi Tag Generator microservice broadcasts UDP dataframes on port 44818.
44. All Sense HAT Pi containers are started using Docker Compose.
45. The HMI Pi Dashboard microservice listens for UDP dataframes on port 8031.
46. The HMI Pi Dashboard microservice posts UDP dataframes on port 8001.
47. The HMI Pi starts the Dashboard microservice via Dockerfile.
48. The laptop Dashboard microservice listens for UDP dataframes on port 8031.
49. The laptop Dashboard microservice posts UDP dataframes on port 8001.
50. The laptop starts the Dashboard microservice via Dockerfile.

## Non-Functional Requirements

### GitHub Version Control
1. The project will use GitHub as a remote repository.
2. Contributors must create a new branch when implementing a new feature.
3. Contributors may only commit their changes if the system has been tested with their implementations.
4. Contributors may only merge their branch with “main” if proper testing has been done, and both members agree.
5. The project will be backed up locally on both team members’ laptops.
6. The project will be backed up daily.

### Unprivileged Socket Port Allocation (1025–65535) 
7. Port 8001 is used for the HMI PI & Laptop Dashboards to send PLC instructions to SHPi PLC Instruction Reader.
8. Port 8010 is used for the SHPi PLC Tag Reader to send PLC tags to SHPi State Formatter.
9. Port 8011 is used for the SHPi PLC Instruction Reader to send PLC instructions to the SHPi Tag Generator.
10. Port 8012 is used for the SHPi Network Monitor to send network state to SHPi State Formatter.
11. Port 8020 is used for the SHPi State Formatter to send state dataframes to the SHPi Log Archiver and Dashboard Updater.
12. Port 8030 is used for SHPi Log Archiver to stream logs to NAS.
13. Port 8031 is used for the SHPi Dashboard Update to send state data to the Laptop and HMI Pi Dashboards.
14. Port 44818 is used for PLC Tag broadcasting between the Station 2 PLC, Robot Arm PLC, and SHPi.

### Managing Network Interfaces (eth0) and (wlan0)
15. The eth0 interface will use a Docker bridge network to virtually bind the port to multiple containers through different port numbers.

### Classroom LAN Confinement
16. Network communication will not go outside of the 192.168.0/16 subnet in the Sandbox_AP network.

### Live Linux CLI Auditability and Verification
17. We’ll use the command 'sudo ss –tuapn' to verify all project sockets and & PIDs.
18. We’ll use the command 'sudo lsof -nP' to verify port ownership.
19. We’ll use the command 'ping <IP Address of Node>' on the SHPi to verify network connections to all other physical nodes.
20. We’ll use the command 'ethtool eth0' to verify physical carrier & speed negotiation.
21. We’ll use the command 'ethtool -S eth0' to diagnose hardware and errors.

### Under 3-Minute Reboot/Launch Reproducibility
22. The system will fully boot in under 30 seconds.
23. The system will boot by running the compose.yaml file on the team member laptop. 
