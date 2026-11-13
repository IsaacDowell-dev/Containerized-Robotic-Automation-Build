from fastapi import APIRouter, Query
from pydantic import BaseModel
from typing import List, Optional, Annotated
from datetime import datetime
from sqlmodel import select, col

from database_api import SessionDep
from models import Log

router = APIRouter(prefix="/query", tags=["Query"])

""" Query parameters for log searching """
class LogQuery(BaseModel):
    source: Optional[str] = None
    status: Optional[str] = None
    start_time: Optional[datetime] = None
    end_time: Optional[datetime] = None
    search: Optional[str] = None # text search in the log message
    id: Optional[str] = None
    limit: int = 100
    offset: int = 0

""" Server result from fetching logs """
class LogQueryResult(BaseModel):
    logs: List[Log]
    total: int
    has_more: bool

""" Search logs with flexible filtering """
@router.get("/search")
async def search_logs(
    session: SessionDep,
    query: LogQuery,
) -> List[Log]:
    # Query thed MariaDB table for relevant log entries
    logs = await get_relevant_logs(query)
    
    # Sort logs by timestamp descending (newest first)
    logs.sort(key=lambda x: x.timestamp, reverse=True)

    # Apply pagination
    total = len(logs)
    paginated = logs[query.offset:query.offset + query.limit]

    return LogQueryResult(
        logs=paginated,
        total=total,
        has_more=(query.offset + query.limit) < total
    )

""" Retrieve all logs in the table """
@router.get("/logs/")
async def read_logs(
    session: SessionDep,
    offset: int = 0,
    limit: Annotated[int, Query(le=100)] = 100,
) -> List[Log]:
    logs = session.exec(
        select(Log).offset(offset).limit(limit)
    ).all()
    return logs

""" Retrieve logs with specific filters """
@router.post("/logs/filter")
async def filter_logs(query: LogQuery) -> LogQueryResult:
    # Get a filtered list of logs from the database
    logs = await get_relevant_logs(query)

    # Sort logs by timestamp
    logs.sort(key=lambda x: x.recorded_at, reverse=True)

    # Apply pagination
    total = len(logs)
    paginated = logs[query.offset:query.offset + query.limit]

    return LogQueryResult(
        logs=paginated,
        total=total,
        has_more=(query.offset + query.limit) < total
    )

""" 
Gets a collection of relevant logs from the MariaDB table based on the query
"""
async def get_relevant_logs(
    session: SessionDep,
    query: LogQuery
) -> List[Log]:
    # Generate the base of the SQL statement
    statement = select(Log)

    # Conditionally add WHERE clauses to the statement if parameters were given
    if query.id is not None:
        statement = statement.where(col(Log.id) == query.id)
    if query.status is not None:
        statement = statement.where(col(Log.status) == query.status)
    if query.source is not None:
        statement = statement.where(col(Log.source) == query.source)
    if query.host_ip is not None:
        statement = statement.where(col(Log.host_ip) == query.host_ip)
    if query.start_time is not None:
        statement = statement.where(col(Log.recorded_at) > query.start_time)
    if query.end_time is not None:
        statement = statement.where(col(Log.recorded_at) < query.end_time)
    if query.search is not None:
        statement = statement.where(col(Log.log_body).contains(query.search))
    
    logs = session.exec(statement)
    return sorted(logs, reverse=True)
