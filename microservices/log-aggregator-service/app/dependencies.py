from fastapi import Depends
from sqlalchemy.ext.asyncio import AsyncSession
from typing import AsyncGenerator

from app.database import sessionmanager

async def get_db() -> AsyncGenerator[AsyncSession, None]:
    async for session in sessionmanager.get_session():
        async with session.begin():
            yield session