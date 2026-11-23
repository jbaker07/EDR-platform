# backend/app/auth.py
#
# Bearer token check for every API route. main.py applies require_token to the
# whole app, so a new route is covered without anyone remembering to add it.
#
# The token comes from EDR_API_TOKEN. With no token the app refuses to start,
# unless EDR_DEV_NO_AUTH=1 is set on purpose for local work. Fail closed.

import hmac
import os

from fastapi import HTTPException, Request, status

TOKEN_ENV = "EDR_API_TOKEN"
DEV_NO_AUTH_ENV = "EDR_DEV_NO_AUTH"
MIN_TOKEN_LENGTH = 32


class AuthConfigError(RuntimeError):
    """Raised at startup when the token setup is unsafe."""


def load_token() -> str | None:
    """Return the configured token, or None only when the dev flag allows it."""
    token = os.environ.get(TOKEN_ENV, "")
    if token:
        if len(token) < MIN_TOKEN_LENGTH:
            raise AuthConfigError(
                f"{TOKEN_ENV} is shorter than {MIN_TOKEN_LENGTH} characters. Make one with: "
                'python -c "import secrets; print(secrets.token_urlsafe(32))"'
            )
        return token
    if os.environ.get(DEV_NO_AUTH_ENV) == "1":
        return None
    raise AuthConfigError(
        f"{TOKEN_ENV} is not set, so the API would be open to anyone who can reach it. "
        f"Set {TOKEN_ENV}, or set {DEV_NO_AUTH_ENV}=1 to run without auth on a dev machine."
    )


def _unauthorized() -> HTTPException:
    return HTTPException(
        status_code=status.HTTP_401_UNAUTHORIZED,
        detail="Missing or invalid bearer token.",
        headers={"WWW-Authenticate": "Bearer"},
    )


async def require_token(request: Request) -> None:
    """FastAPI dependency: reject the request unless it carries the right token."""
    expected = request.app.state.api_token
    if expected is None:
        return  # dev mode, set on purpose with EDR_DEV_NO_AUTH=1
    scheme, _, supplied = request.headers.get("Authorization", "").partition(" ")
    if scheme.lower() != "bearer" or not supplied:
        raise _unauthorized()
    # compare_digest takes the same time wherever the first wrong byte is, so the
    # response time does not leak how much of a guess was right.
    if not hmac.compare_digest(supplied.encode("utf-8"), expected.encode("utf-8")):
        raise _unauthorized()
