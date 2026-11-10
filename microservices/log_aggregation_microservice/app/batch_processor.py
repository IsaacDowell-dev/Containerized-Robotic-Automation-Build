import asyncio
from typing import List
from datetime import datetime, UTC
from dataclasses import dataclass
import json

from ingestion import LogEntry

""" Configuration variables for the batch processor """
@dataclass
class ProcessorConfig:
    batch_size: int = 1000 # Max logs per batch
    flush_interval: float = 5.0 # Seconds before each flush
    # Needs to define API for querying the MariaDB table

""" Processes logs from the buffer in batches """
class BatchProcessor:
    def __init__(self, buffer: asyncio.Queue, config: ProcessorConfig):
        self.buffer = buffer
        self.config = config
        self.current_batch: List[LogEntry] = []
        self.last_flush = datetime.now(UTC)
        self._running = False

    """ Start the batch processor """
    async def start(self):
        self._running = True

        # Connect to MariaDB

        # Run processor loop
        await asyncio.gather(
            self._process_loop(),
            self._flush_timer(),
        )

    """ Stop the processor """
    async def stop(self):
        self._running = False

    """ Main process loop: reading from the buffer """
    async def _process_loop(self):
        while self._running:
            try:
                # Wait for log entry with timeout
                log  = await asyncio.wait_for(
                    self.buffer.get(),
                    timeout=1.0
                )
                
                self.current_batch.append(log)

                # Flush the batch if full
                if len(self.current_batch) >= self.config.batch_size:
                    await self._flush_batch()

            except asyncio.TimeoutError:
                continue
            except Exception as e:
                print(f"Batch Processing Error: {e}")

    """ Periodic flush timer """
    async def _flush_timer(self):
        while self._running:
            await asyncio.sleep(self.config.flush_interval)

            # Flush if there are currently pending logs
            if self.current_batch:
                await self._flush_batch()

    """ Write current batches to the MariaDB database """
    async def _flush_batch(self):
        if not self.current_batch:
            return

        # Grab the batch and reset
        batch = self.current_batch
        self.current_batch = []
        self.last_flush = datetime.now(UTC)

        # Establish connection to the database

        # Generate query

        # Execute query on the database

        print(f"Flushed {len(batch)} logs to the database.")
