from __future__ import annotations

from datetime import datetime, UTC

from sqlalchemy import DateTime, Integer, String, Text
from sqlalchemy.orm import Mapped, mapped_column
from typing import List, Optional

from app.database import Base

# SQLAlchemy Logs table declaration
class Log(Base):
    __tablename__ = "logs"

    # Attributes with "index=True" are indexed for quicker searching in the db
    id: Mapped[int] = mapped_column(Integer, primary_key=True, index=True)  # Primary key of the table
    recorded_at: Mapped[datetime] = mapped_column(                          # Datetime that the log was recorded at
        DateTime(timezone=True),    
        default=lambda: datetime.now(UTC),                                  # If not provided, append the current time
    )   
    level: Mapped[str] = mapped_column(String(10), nullable=False)          # Log level
    service: Mapped[str] = mapped_column(String(25), nullable=False)        # Name of microservice
    log_body: Mapped[str] = mapped_column(Text, nullable=False) 
    host: Mapped[str] = mapped_column(String(15), nullable=False)           # IP address of service that sent the message