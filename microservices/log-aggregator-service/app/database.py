from __future__ import annotations

from typing import Annotated, AsyncGenerator, Optional

from sqlalchemy.ext.asyncio import (
    AsyncEngine,
    AsyncSession,
    async_sessionmaker,
    create_async_engine
)
from sqlalchemy.orm import DeclarativeBase
from sqlalchemy.pool import AsyncAdaptedQueuePool
from sqlalchemy.sql import text

from app.config import DB_URL 

class SessionManager:
    """ Manages async db sessions with connection pooling """
    def __init__(self) -> None:
        self.engine: Optional[AsyncEngine] = None
        self.session_factory: Optional[async_sessionmaker[AsyncSession]]

    def init_db(self) -> None:
        """ Init the database engine and session factory """
        self.engine = create_async_engine(
            DB_URL,
            poolclass=AsyncAdaptedQueuePool,
            pool_size=5,
            max_overflow=2,
            pool_pre_ping=True,
            pool_recycle=5,
            echo=True,
        )

        self.session_factory = async_sessionmaker(
            self.engine,
            expire_on_commit=False,
            autoflush=False,
            class_=AsyncSession,
        )

    async def close(self) -> None:
        """ Dispose the database engine """
        if self.engine:
            await self.engine.dispose()

    async def get_session(self) -> AsyncGenerator[AsyncSession, None]:
        """ Yield a database session with the correct schema set """
        if not self.session_factory:
            raise RuntimeError("Database session factory not initialized.")
    
        async with self.session_factory() as session:
            try:
                yield session
            except Exception as e:
                await session.rollback()
                raise RuntimeError(f"Database session error: {e!r}") from e

# Base class for declarative models used for buulding tables
class Base(DeclarativeBase):
    pass

sessionmanager = SessionManager()
