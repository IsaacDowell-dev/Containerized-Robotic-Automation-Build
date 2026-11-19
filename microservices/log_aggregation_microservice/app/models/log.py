from datetime import datetime
from sqlmodel import SQLModel, Field
from pydantic import BaseModel
from typing import List, Optional

# Log entry structure for the database
class Log(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    recorded_at: datetime = Field(index=True)
    status: str = Field(index=True)
    source: str = Field(index=True)
    log_body: str
    host_ip: str = Field(index=True)

# Query structure for requesting logs from the database 
class LogQuery(BaseModel):
    id: Optional[str] = None                # Log UUID
    start_time: Optional[datetime] = None   # Start of a date range of logs
    end_time: Optional[datetime] = None     # End of a time range of lags
    status: Optional[str] = None            # Log level
    source: Optional[str] = None            # Source service of log
    search: Optional[str] = None            # Text search for log-body content  
    limit: int = 100                        # Max num of logs to be fetched
    offset: int = 0                         # Pagination offset of log response

# Server response structure for log requests
class LogQueryResult(BaseModel):
    logs: List[Log]
    total: int
    has_more: bool
