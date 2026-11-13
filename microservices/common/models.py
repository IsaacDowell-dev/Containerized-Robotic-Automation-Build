from datetime import datetime
from sqlmodel import Field, SQLModel

class Log(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    recorded_at: datetime = Field(index=True)
    status: str = Field(index=True)
    source: str = Field(index=True)
    log_body: str
    host_ip: str = Field(index=True)
