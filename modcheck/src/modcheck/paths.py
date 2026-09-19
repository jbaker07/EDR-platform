"""Filesystem layout. One place that knows where things live."""
from __future__ import annotations

import os
from pathlib import Path

_ENV = "MODCHECK_ROOT"


def project_root() -> Path:
    """Root of the ModCheck project (the directory holding packs/ and schemas/)."""
    override = os.environ.get(_ENV)
    if override:
        return Path(override).resolve()
    return Path(__file__).resolve().parents[2]


def packs_dir() -> Path:
    return project_root() / "packs"


def schemas_dir() -> Path:
    return project_root() / "schemas"


def evidence_cache() -> Path:
    """Cached upstream bytes. Gitignored: re-fetchable, often not redistributable."""
    return Path(os.environ.get("MODCHECK_EVIDENCE_CACHE", project_root() / "evidence_cache"))


def workspaces_dir() -> Path:
    """Scratch space for generated projects and build runs."""
    return Path(os.environ.get("MODCHECK_WORKSPACES", project_root() / "build_workspaces"))


GAMES: tuple[str, ...] = (
    "minecraft",
    "sims4",
    "skyrimse",
    "fallout4",
    "cyberpunk2077",
    "bg3",
    "falloutnv",
    "stardewvalley",
    "projectzomboid",
    "rimworld",
)

CAPABILITIES: tuple[str, ...] = (
    "docs_guidance",
    "project_setup",
    "source_edit",
    "package_inspection",
    "dependency_analysis",
    "interaction_analysis",
    "build_execution",
    "runtime_testing",
    "release_maintenance",
)
