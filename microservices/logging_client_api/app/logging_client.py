# Client library for sending logs to the aggregation service
import requests
from typing import Optional
from datetime import datetime, UTC
from dataclasses import dataclass
from enum import Enum
import queue
import threading
import atexit

""" Enumerator describing different log levels that are accepted """
class LogLevel(str, Enum):
    DEBUG = "debug"
    INFO = "info"
    WARNING = "warning"
    ERROR = "error"
    CRITICAL = "critical"

""" Configuration variables for logging client """
@dataclass
class LogConfig:
    service_name: str
    aggregation_url: str
    batch_size: int = 100
    flush_interval: float = 5.0
    max_queue_size: int = 10000

""" Client object for sending logs to the aggregation service """
class LogAggregationClient:
    def __init__(self, config: LogConfig):
        self.config = config
        self._queue = queue.Queue(maxsize=config.max_queue_size)
        self._running = True

        # Start backgroynd flushing thread
        self._flush_thread = threading.Thread(target=self._flush_loop, daemon=True)
        self._flush_thread.start()

        # Ensure flush on exit
        atexit.register(self.close)
    
    """ Log a message; queues the log for sending and is non-blocking """
    def log(
        self,
        timestamp: datetime,
        level: LogLevel,
        message: str,
        **attributes
    ):
        entry = {
            "timestamp": timestamp,
            "level": level.value,
            "service": self.config.service_name,
            "message": message,
            "attributes": attributes,
        }

        try:
            self._queue.put_nowait(entry)
        except queue.Full:
            # Drop logs if the queue is full
            pass

    """ Top-level logging library functions """
    def debug(self, message: str, **kwargs):
        self.log(LogLevel.DEBUG, message, **kwargs)

    def info(self, message: str, **kwargs):
        self.log(LogLevel.INFO, message, **kwargs)

    def warning(self, message: str, **kwargs):
        self.log(LogLevel.WARNING, message, **kwargs)

    def error(self, message: str, **kwargs):
        self.log(LogLevel.ERROR, message, **kwargs)

    def critical(self, message: str, **kwargs):
        self.log(LogLevel.CRITICAL, message, **kwargs)

    """ Background thread that flushes the logs periodically """
    def _flush_loop(self):
        while self._running():
            self._flush_batch()
            threading.Event().wait(self.config.flush_interval)

    """ Flush pending logs to the aggregator """
    def _flush_batch(self):
        batch = []

        # Drain the queue up to batch size
        while len(batch) < self.config.batch_size:
            try:
                entry = self._queue.get_nowait()
                batch.append(entry)
            except queue.Empty:
                break

        if not batch:
            return

        # Send batch to the aggregator
        try:
            response = requests.post(
                f"{self.config.aggregator_url}/ingest/batch",
                json={"logs": batch},
                timeout=5.0,
            )

            if response.status != 200:
                # Re-queue failed logs (best effort approach)
                for entry in batch:
                    try:
                        self._queue.put_nowait(entry)
                    except queue.Full:
                        break

        except requests.RequestException:
            # Network error - re-queue logs
            for entry in batch:
                try:
                    self._queue.put_nowait(entry)
                except queue.Full:
                    break

    """ Flush remaining logs and stop the client """
    def close(self):
        self._running = False
        
        # Final flush
        self._flush_batch()
