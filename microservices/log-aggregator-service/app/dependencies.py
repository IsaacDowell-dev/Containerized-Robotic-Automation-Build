from fastapi import Depends
from sqlalchemy.ext.asyncio import AsyncSession
from typing import Annotated

from app.database.session import get_async_session

# Type alias for DRY dependency injection
AsyncSessionDep = Annotated[AsyncSession, Depends(get_async_session)]