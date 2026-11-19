from fastapi import APIRouter, Request, HTTPException, status
from sqlalchemy import select
from datetime import datetime, UTC
# from typing import List
# import asyncio

import app.models as models
from app.schemas.logs import (
    LogCreate, 
    LogResponse, 
    #PaginatedLogResponse # SQLModel log standard object format
)
from app.dependencies import AsyncSessionDep

# Router instance to mount the ingest route to the main.py app
router = APIRouter()

""" Logging endpoint for fast, low-volume logging"""
@router.post(
    "",                                     # Absolute route: /api/ingest
    response_model=LogResponse,             # Response format for client
)
async def ingest_single_log(log: LogCreate, db: AsyncSessionDep): 
    # Create a new log object and add it to the session
    new_log = await models.Log(
        recorded_at=log.recorded_at,
        level=log.level,
        service=log.service,
        log_body=log.log_body,
        host=log.host,
    )

    # Commit the transaction, and get updated data from db
    await db.commit()
    await db.refresh(new_log)

    return new_log

""" Logging endpoint for ingesting a batch of logs"""
@router.post("/batch")
async def ingest_batch_logs(batch: List[Log], request: Request):
    pass
