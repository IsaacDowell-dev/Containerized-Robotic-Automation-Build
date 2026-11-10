from fastapi import APIRouter, Query
from pydantic import BaseModel
from typing import List, Optional
from datetime import datetime, timedelta, UTC
import asyncio
import json
import os
import mariadb

""" Database Connection Details """
db_config = {
    'user': os.getenv('MARIA_DB_USER'),
    'password': os.getenv('MARIA_DB_PASSWORD'),
    'host': os.getenv('MARIA_DB_HOST'),
    'database': os.getenv('MARIA_DB_NAME'),
    'port': os.getenv('MARIA_DB_PORT', 3306)
}

from ingestion import LogEntry, LogLevel

router = APIRouter(prefix="/query", tags=["Query"])

""" Query parameters for log searching """
class LogQuery(BaseModel):
    service: Optional[str] = None
    level: Optional[str] = None
    start_time: Optional[datetime] = None
    end_time: Optional[datetime] = None
    search: Optional[str] = None # text search in the log message
    limit: int = 100
    offset: int = 0

""" Result of a log query """
class LogQueryResult(BaseModel):
    logs: List[LogEntry]
    total: int
    has_more: bool

""" Search logs with flexible filtering """
@router.post("/search")
async def search_logs(query: LogQuery) -> LogQueryResult:
    # Query the MariaDB table for relevant log entries
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

""" 
Gets a collection of relevant logs from the MariaDB table based on the query
Uses timestamps
"""
async def get_relevant_logs(query: LogQuery):
    logs = []

    # Connect to the database and establish a cursor
    db_conn = mariadb.connect(**db_config)
    cursor = db_conn.cursor()

    # Define the SQL Query

    # Return a sorted list of logs
    return sorted(logs, reverse=True) # Newest first

""" Gets the latest logs uploaded to the database for a specific service """
@router.get("/tail/{service}")
async def tail_logs(
    service: str,
    lines: int = Query(default=100, le=1000)
) -> List[LogEntry]:
    query = LogQuery(service=service, limit=lines)
    result = await search_logs(query)
    return result.logs
