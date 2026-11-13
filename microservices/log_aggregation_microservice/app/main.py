# Main entry point for the logging aggregation service
from fastapi import FastAPI
from contextlib import asynccontextmanager
import asyncio

from ingestion import app as ingestion_app, get_buffer
from processor import BatchProcessor, ProcessorConfig
from query import router as query_router

# Singelton processor instance
processor: BatchProcessor = None

""" Manage the application lifecycle """
@asynccontextmanager
async def lifespan(app: FastAPI):
    global processor

    # Start the batch processor
    buffer = await get_buffer()
    config = ProcessorConfig(
        batch_size=1000,
        flush_interval=5.0,
    )

    # Establish the database engine and start a session

    processor = BatchProcessor(buffer, config)

    # Start the processor in the background
    processor_task = asyncio.create_task(processor.start())

    yield

    # Shutdown
    processor.stop()
    processor_task.cancel()

# Create main app
app = FastAPI(title="Log Aggregation Service", lifespan=lifespan)

# Mount routers
app.include_router(ingestion_app.router)
app.include_router(query_router)

""" Health check endpoint """
async def health():
    buffer = await get_buffer()
    return {
        "status": "healthy",
        "queue_size": buffer.qsize()
    }

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)

