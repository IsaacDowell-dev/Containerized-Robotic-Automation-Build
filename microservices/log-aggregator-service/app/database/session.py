from typing import Annotated, AsyncGenerator
from fastapi import Depends

from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker, create_async_engine
from sqlalchemy.orm import DeclarativeBase

from app.core.config import DB_URL 

# Establish the DB engine and a AsyncSession type
engine = create_async_engine(DB_URL, echo=True)

# Factory method that yields an AsyncSession instance
AsyncSessionLocal = async_sessionmaker(
    engine,
    class_=AsyncSession,    # Class type of the factory product
    expire_on_commit=False  # Turn off lazy loading
)

# Base class for declarative models used for buulding tables
class Base(DeclarativeBase):
    pass

# Dependency injection function that allows a route access to the database
async def get_async_session() -> AsyncGenerator[AsyncSession, None]:
    async with AsyncSessionLocal() as session:
        # Handle commit, rollback, and cleanup
        try:
            yield session
            await session.commit()
        except Exception:
            await session.rollback()
            raise
        finally:
            await session.close()
