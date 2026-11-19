from fastapi import APIRouter, Query
from typing import List, Annotated
from sqlmodel import select, col

from app.models.log import Log, LogQuery, LogQueryResult
from app.db.session import SessionDep

""" Router endpoint for these functions """ 
router = APIRouter(prefix="/query", tags=["Query"])

""" Retrieve limit number logs in the table """
@router.get("/logs/")
async def read_logs(
    session: SessionDep,
    offset: int = 0,
    limit: Annotated[int, Query(le=100)] = 100,
) -> List[Log]:
    # Execute a select query for all logs in the table
    logs = await session.exec(
        select(Log).offset(offset).limit(limit)
    ).all()

    return logs

""" Retrieve logs with specific filters """
@router.post("/logs/filter")
async def filter_logs(session: SessionDep, query: LogQuery) -> LogQueryResult:
    # Get a filtered list of logs from the database
    logs = await get_relevant_logs(session, query)

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
    
    logs = await session.exec(statement)
    return sorted(logs, reverse=True)
