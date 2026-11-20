from fastapi import APIRouter, Depends, Request, HTTPException, status
from sqlalchemy import select
from sqlalchemy.ext.asyncio import AsyncSession
from datetime import datetime, UTC
from typing import List
# import asyncio

import app.models as models
from app.schemas import (
    LogCreate, 
    LogResponse, 
    BatchLogResponse,
    #PaginatedLogResponse # SQLModel log standard object format
)
from app.dependencies import get_db

# Router instance to mount the ingest route to the main.py app
router = APIRouter()

""" Logging endpoint for fast, low-volume logging"""
@router.post(
    "",                                     # Absolute route: /api/ingest
    response_model=LogResponse,             # Response format for client
)
async def ingest_single_log(
    log: LogCreate,
    db: AsyncSession = Depends(get_db)
): 
    # Create a new log object and add it to the session
    new_log = models.Log(
        recorded_at=log.recorded_at,
        level=log.level,
        service=log.service,
        log_body=log.log_body,
        host=log.host,
    )

    # Commit the transaction, and get updated data from db
    db.add(new_log)
    await db.commit()
    await db.refresh(new_log)

    return new_log

""" Logging endpoint for uploading a batch of logs at once """
@router.post(
    "/batch",
    response_model=BatchLogResponse,
)
async def ingest_batch_logs(
    logs: List[LogCreate],
    db: AsyncSession = Depends(get_db)
):
    # Create pydantic models of each log received
    new_logs = []
    for log in logs:
        new_log = models.Log(
            recorded_at=log.recorded_at,
            level=log.level,
            service=log.service,
            log_body=log.log_body,
            host=log.host,
        )
        
        new_logs.append(new_log)

    # Add and commit all of the new log objects to the session, then retrun the refreshed response
    db.add_all(new_logs)
    await db.commit()
    await db.refresh(new_logs)

    return BatchLogResponse(
        logs: new_logs,
    )