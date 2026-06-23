"""Shared application loggers.

This module was referenced by the rules and correlation engines
(``from app.logs.logger import rules_logger``) but was never committed —
the over-broad ``logs/`` .gitignore rule swallowed the whole package, which
broke the app at import time. It is restored here as a thin, dependency-free
wrapper around the stdlib ``logging`` module.
"""

import logging

_FORMAT = "%(asctime)s [%(levelname)s] %(name)s: %(message)s"


def _make_logger(name: str, level: int = logging.INFO) -> logging.Logger:
    logger = logging.getLogger(name)
    if not logger.handlers:
        handler = logging.StreamHandler()
        handler.setFormatter(logging.Formatter(_FORMAT))
        logger.addHandler(handler)
        logger.setLevel(level)
        logger.propagate = False
    return logger


# Imported by app.rules.rules_engine and app.memory.correlation_engine.
rules_logger = _make_logger("edr.rules")
