# Containerized-Robotic-Automation-Build

# Purpose
The purpose of CRAB is to automate the deployment of robotic control systems. The software will have containerized systems for sending signals, collecting logs, displaying logs, and reading input. The system will allow for quick deployment of software connecting PLCs and robotic equipment. Additionally, it will support logging through NAS and displaying logs.

# Scope
HMI Raspberry Pi Role: Visual dashboard for the current state of the PLC, robot arm, and network connections. Also acts as a secondary controller for the robot arm.

Sense HAT Raspberry Pi Role: Bridge between all other connected nodes (HMIPi, Laptop, Robot Arm, PLC), and physical temperature sensor.

Remote Storage: In-lab NAS server for uploading program logs from the SHPi.

Additional Nodes: Robotic Arm and Connected PLC (R2D2), team-member laptop, and station 2 CompactLogix PLC.

# Out-of-Scope Boundary
Due to time limitations, we plan on excluding the implementation of Docker Swarm for automated container deployment to the Raspberry Pis, and will instead opt for a BASH script to automatically SSH into each Pi and start their containers.
