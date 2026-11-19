""" CONFIGURATION OR ENVIRONMENT VARIABLES FOR THE APP """
import os

# Database variables
# Temporary placement of actual values
DB_USER = os.getenv('DB_USER', 'Sandbox2')
DB_PASS = os.getenv('DB_PASS', 'Data_Base_Rocks_101')
DB_HOST = os.getenv('DB_HOST', '192.168.5.254')
DB_PORT = os.getenv('DB_PORT', '3306')
DB_NAME = os.getenv('DB_NAME', 'Logs')

DB_URL = (
    f"mariadb+aiomysql://{DB_USER}:{DB_PASS}"
    f"@{DB_HOST}:{DB_PORT}"
    f"/{DB_NAME}"
)
