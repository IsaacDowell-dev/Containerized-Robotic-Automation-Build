from logging_client_api.client import LogConfig, LogAggregationClient
from datetime import datetime, UTC
import time

if __name__ == "__main__":
    # Create the client
    config = LogConfig(
        service_name="test-service",
        aggregation_url="http://localhost:8000/logging-ingest",
    )

    logger = LogAggregationClient(config)

    # Upload a multiple test logs
    for i in range(5):
        logger.info(
            datetime.now(UTC).isoformat(),
            "Testing logging client functionality.",
            '127.0.0.1',
        )
        time.sleep(0.1)

    # Clean shutdown
    logger.close()
