# Main entry point for the logging aggregation service
from fastapi import FastAPI, HTTPException, Request, status
from sqlalchemy import select
from contextlib import asynccontextmanager
import asyncio

from app.routers import ingestion
from app.dependencies import AsyncSessionDep
from app.database.session import engine
# from app.routers.query import router as query_router
# from app.services.processor import BatchProcessor, ProcessorConfig

# Singelton processor instance
# processor: BatchProcessor = None

""" Manage the application lifecycle """
@asynccontextmanager
async def lifespan(app: FastAPI):
    # global processor

    # Obtain the buffer from the ingestion api
    # buffer = await get_buffer()

    # Define contant configurations for the batch processor
    # config = ProcessorConfig(
        # batch_size=1000,
        # flush_interval=5.0,
    # )

    # Start the database engine
    async with engine.begin() as conn:
        # Create database tables if they don't exist yet
        await conn.run_async(Base.metadata.create_all)

    # Initialize the BatchProcessor object 
    # processor = BatchProcessor(buffer, config)

    # Start the processor in the background
    # processor_task = asyncio.create_task(processor.start())

    yield

    # Shutdown
    # processor.stop()
    # processor_task.cancel()

# Create main app
app = FastAPI(title="Log Aggregation Service", lifespan=lifespan)

# Mount routers with their api prefixes
app.include_router(ingestion.router, prefix="/api/ingest", tags=["ingest"])
# app.include_router(query_router, prefix="api/query", tags=["query"])

""" Health check endpoint """
async def health(db: AsyncSessionDep):
    # TODO: Implement database query for health check
    # buffer = await get_buffer()

    try:
        await db.execute(select(1))
    except Exception as exc:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Database unavailable",
        ) from exc

    return {
        "status": "healthy",
        # "queue_size": buffer.qsize()
    }

# if __name__ == "__main__":
#     import uvicorn
#     uvicorn.run(app, host="0.0.0.0", port=8000)