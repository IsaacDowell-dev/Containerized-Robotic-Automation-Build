# Main entry point for the logging aggregation service
from fastapi import FastAPI, Depends, HTTPException, Request, status
from sqlalchemy import select
from sqlalchemy.ext.asyncio import AsyncSession
from contextlib import asynccontextmanager
import asyncio

from app.routers import ingestion
from app.dependencies import get_db
from app.database import sessionmanager, Base
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

    # Init the database session manager
    sessionmanager.init_db()

    # Start the database engine
    async with sessionmanager.engine.begin() as conn:
        # Create database tables if they don't exist yet
        await conn.run_sync(Base.metadata.create_all)
    yield

    # Initialize the BatchProcessor object 
    # processor = BatchProcessor(buffer, config)

    # Start the processor in the background
    # processor_task = asyncio.create_task(processor.start())

    yield

    # Shutdown
    await sessionmanager.close()
    # processor.stop()
    # processor_task.cancel()

# Create main app
app = FastAPI(title="Log Aggregation Service", lifespan=lifespan)

# Mount routers with their api prefixes
app.include_router(ingestion.router, prefix="/api/ingest", tags=["ingest"])
# app.include_router(query_router, prefix="api/query", tags=["query"])

""" Health check endpoint """
async def health(db: AsyncSession = Depends(get_db)):
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