from datetime import datetime

from pydantic import BaseModel, Field, ConfigDict

""" LOG SCHEMA DEFINITIONS """

# Base level of a Log's data structure
class LogBase(BaseModel):
    recorded_at: datetime
    level: str = Field(min_length=1, max_length=10)
    service: str = Field(min_length=1, max_length=10)
    log_body: str = Field(min_length=1)
    host: str = Field(min_length=7, max_length=15)

# Model used for creating a log; no different from base
class LogCreate(LogBase):
    pass

# Server response model when querying a log
class LogResponse(LogBase):
    # model_config = ConfigDict(from_attributes=True)
    # id: int
    pass

# Paginated collection of logs
# class PaginatedLogResponse(BaseModel):
#     logs = list[LogResponse]
#     total: int
#     skip: int
#     limit: int
#     has_more: bool