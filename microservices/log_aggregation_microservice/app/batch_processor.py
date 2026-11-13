import asyncio
from typing import List
from datetime import datetime, UTC
from dataclasses import dataclass

from models import Log
from database_api import SessionDep

""" Configuration variables for the batch processor """
@dataclass
class ProcessorConfig:
    batch_size: int = 1000 # Max logs per batch
    flush_interval: float = 5.0 # Seconds before each flush

""" Processes logs from the buffer in batches """
class BatchProcessor:
    def __init__(self, buffer: asyncio.Queue, config: ProcessorConfig, session: SessionDep):
        self.buffer = buffer
        self.config = config
        self.current_batch: List[Log] = []
        self.last_flush = datetime.now(UTC)
        self.session = session
        self._running = False

    """ Start the batch processor """
    async def start(self):
        print("[LOG AGG] Starting batch processor service...")
        self._running = True

        # Run processor loop
        await asyncio.gather(
            self._process_loop(),
            self._flush_timer(),
        )

    """ Stop the processor """
    async def stop(self):
        print("[LOG AGG] Stopping batch processor service...")
        self._running = False

    """ Main process loop: reading from the buffer """
    async def _process_loop(self):
        print("[LOG AGG] Starting batch processing loop...")
        while self._running:
            try:
                # Wait for log entry with timeout
                log  = await asyncio.wait_for(
                    self.buffer.get(),
                    timeout=1.0
                )
               
                # Append the incoming log to the current batch
                self.current_batch.append(log)

                # Flush the batch if full
                if len(self.current_batch) >= self.config.batch_size:
                    await self._flush_batch()

            except asyncio.TimeoutError:
                print("[LOG AGG] Batch processor timed out waiting for a log; continuing process loop...")
                continue
            except Exception as e:
                print(f"[LOG AGG] Batch processor encountered an exception; halting process loop...\n{e}")

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
        print("[LOG AGG] Flushing current batch of logs...")
        
        # Grab the batch and reset
        batch: List[Log] = self.current_batch
        self.current_batch = []
        self.last_flush = datetime.now(UTC)

        # Add all logs to the database session
        self.session.add_all(batch)
    
        # Upload the batch of logs to the database
        self.session.commit()

        print(f"Flushed {len(batch)} logs to the database.")
