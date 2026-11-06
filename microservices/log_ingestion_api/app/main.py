from fastapi import FastAPI, Request, HTTPException
from pydantic import BaseModel, Field
from enum import Enum
from datetime import datetime, UTC
from typing import List
import asyncio

""" Return the current UTC datetime"""
def utc_now() -> datetime:
    return datetime.now(UTC)

""" Enumerator for each kind of log level that can be thrown"""
class LogLevel(str, Enum):
    DEBUG = "debug"
    INFO = "info"
    WARNING = "warning"
    ERROR = "error"
    CRITICAL = "critical"

""" Normalized log entry format for all services"""
class LogEntry(BaseModel):
    timestamp: datetime = Field(default_factory=utc_now)
    level: LogLevel = LogLevel.INFO
    service: str    # name of the service that uploaded the log
    log_body: str   # main content of the log
    host: str

""" Batch of normalized logs"""
class LogBatch(BaseModel):
    logs: List[LogEntry]

""" Singleton buffer to store incoming logs for batch uploads to the DB"""
log_buffer: asyncio.Queue = None

""" Access method for the buffer singleton"""
async def get_buffer() -> asyncio.Queue:
    global log_buffer
    # Create buffer if not instantiated
    if log_buffer is None:
        log_buffer = asyncio.Queue(maxsize=100000)
    return log_buffer

app = FastAPI(title="Log Ingestion API")

""" Logging endpoint for fast, low-volume logging"""
@app.post("/logging-ingest")
async def ingest_single_log(log: LogEntry, request: Request): 
    # 1. Acquire the buffer
    buffer = await get_buffer()

    # 2. Push the incoming log into a buffer for batch processing
    try:
        buffer.put_nowait(log) # puts item into queue without blocking
        return {"status": "accepted", "queue_size": buffer.qsize()}
    except asyncio.QueueFull:
        # Buffer is full, so apply backpressure
        raise HTTPException(status_code=503, detail="buffer_full")

""" Logging endpoint for ingesting a batch of logs"""
@app.post("/logging-ingest/batch")
async def ingest_batch_logs(batch: LogBatch, request: Request):
    # 1. Acquire the buffer
    buffer = await get_buffer()
    accepted = 0

    # 2. Iterate through the batch of logs, and put them into the buffer
    for log in batch.logs:
        try:
            buffer.put_nowait(log)
            accepted += 1
        except asyncio.QueueFull:
            break

    # 3. Return a response to the sender
    return {
        "status": "accepted",
        "accepted": accepted,
        "total": len(batch.logs),
        "queue_size": buffer.qsize(),
    }
