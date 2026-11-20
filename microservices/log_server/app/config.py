""" CONFIGURATION OR ENVIRONMENT VARIABLES FOR THE APP """
import os
from dotenv import load_dotenv

# Load env variables from .env
load_dotenv()

# Database variables
# Temporary placement of actual values
DB_USER = os.getenv('DB_USER')
DB_PASS = os.getenv('DB_PASS')
DB_HOST = os.getenv('DB_HOST')
DB_PORT = os.getenv('DB_PORT', '3306')
DB_NAME = os.getenv('DB_NAME')

DB_URL = (
    f"mariadb+aiomysql://{DB_USER}:{DB_PASS}"
    f"@{DB_HOST}:{DB_PORT}"
    f"/{DB_NAME}"
)