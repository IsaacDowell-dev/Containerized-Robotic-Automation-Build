from sqlmodel import Session, create_engine
from fastapi import Depends
from typing import Annotated
import os

# Database server URL formed from ENV variables
DATABASE_URL = (
    f"mariadb+aiomysql://{os.getenv('DB_USER')}:{os.getenv('DB_PASSWORD')}"
    f"@{os.getenv('DB_HOST')}:{os.getenv('DB_PORT', '3306')}"
    f"/{os.getenv('DB_NAME')}"
)
CONNECT_ARGS = {"check_same_thread": False}

# Database engine instance
engine = create_engine(DATABASE_URL, connect_args=CONNECT_ARGS)

# Database Session dependency
def get_session():
    with Session(engine) as session:
        yield session

SessionDep = Annotated[Session, Depends(get_session)]
