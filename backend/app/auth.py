# backend/app/auth.py
#
# Bearer token check for every API route. main.py applies require_token to the
# whole app, so a new route is covered without anyone remembering to add it.
#
# The token comes from EDR_API_TOKEN_FILE (an owner-only file, re-read when it
# changes, so the token can be rotated without a restart) or EDR_API_TOKEN. With no
# token the app refuses to start, unless EDR_DEV_NO_AUTH=1 is set on purpose for
# local work. Fail closed.

import hmac
import os
import sys
from pathlib import Path

from fastapi import HTTPException, Request, status

TOKEN_ENV = "EDR_API_TOKEN"
TOKEN_FILE_ENV = "EDR_API_TOKEN_FILE"
DEV_NO_AUTH_ENV = "EDR_DEV_NO_AUTH"
MIN_TOKEN_LENGTH = 32


class AuthConfigError(RuntimeError):
    """Raised at startup when the token setup is unsafe."""


class TokenFile:
    """A token kept in a file. current() checks the file on every request (one stat)
    and reads it again when it changed. A changed file that is not owner-only or holds
    a short token is refused, and the token from before stays in use."""

    def __init__(self, path: str | Path):
        self.path = Path(path)
        self._sig = None
        self._token = self._read()
        self._sig = self._stat_sig()

    def _stat_sig(self):
        st = os.stat(self.path)
        return (st.st_ino, st.st_mtime_ns, st.st_size)

    def _read(self) -> str:
        st = os.stat(self.path)
        if st.st_mode & 0o077:
            raise AuthConfigError(f"{TOKEN_FILE_ENV} {self.path} can be read by group or others, chmod 600 it")
        lines = self.path.read_text().splitlines()
        token = lines[0].strip() if lines else ""
        if len(token) < MIN_TOKEN_LENGTH:
            raise AuthConfigError(f"the token in {self.path} is shorter than {MIN_TOKEN_LENGTH} characters")
        return token

    def current(self) -> str:
        try:
            sig = self._stat_sig()
            if sig != self._sig:
                self._sig = sig
                self._token = self._read()
                print(f"API token rotated from {self.path}", file=sys.stderr)
        except (OSError, AuthConfigError) as e:
            print(f"WARNING: {e}, keeping the current token", file=sys.stderr)
        return self._token


def load_token() -> str | TokenFile | None:
    """Return the configured token (or its file), or None only when the dev flag allows it."""
    path = os.environ.get(TOKEN_FILE_ENV, "")
    if path:
        return TokenFile(path)
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
    if isinstance(expected, TokenFile):
        expected = expected.current()
    scheme, _, supplied = request.headers.get("Authorization", "").partition(" ")
    if scheme.lower() != "bearer" or not supplied:
        raise _unauthorized()
    # compare_digest takes the same time wherever the first wrong byte is, so the
    # response time does not leak how much of a guess was right.
    if not hmac.compare_digest(supplied.encode("utf-8"), expected.encode("utf-8")):
        raise _unauthorized()
