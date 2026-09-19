"""YAML loading with YAML 1.2 boolean semantics.

PyYAML implements YAML 1.1, where the bare words ``yes``, ``no``, ``on`` and
``off`` resolve to booleans. Our records use ``yes``/``no`` as enum values --
``detectable: yes``, ``redistribute: no`` -- so under YAML 1.1 they silently
become ``True``/``False`` and fail validation for a reason that has nothing to
do with the author's intent.

YAML 1.2, the current specification, recognises only ``true``/``false``. This
loader follows that, so records read the way they are written.
"""
from __future__ import annotations

from pathlib import Path
from typing import Any

import yaml


class ModCheckLoader(yaml.SafeLoader):
    """SafeLoader with YAML 1.2 boolean resolution."""


# Re-register the bool resolver for only the YAML 1.2 spellings.
_resolvers = ModCheckLoader.yaml_implicit_resolvers.copy()
for _key, _mappings in _resolvers.items():
    _resolvers[_key] = [(tag, regex) for tag, regex in _mappings
                        if tag != "tag:yaml.org,2002:bool"]
ModCheckLoader.yaml_implicit_resolvers = _resolvers
ModCheckLoader.add_implicit_resolver(
    "tag:yaml.org,2002:bool",
    __import__("re").compile(r"^(?:true|True|TRUE|false|False|FALSE)$"),
    list("tTfF"))


def safe_load(stream: Any) -> Any:
    return yaml.load(stream, Loader=ModCheckLoader)


def load_path(path: Path) -> Any:
    with Path(path).open("r", encoding="utf-8") as fh:
        return safe_load(fh)


def dump(data: Any, **kwargs: Any) -> str:
    kwargs.setdefault("sort_keys", False)
    kwargs.setdefault("allow_unicode", True)
    kwargs.setdefault("default_flow_style", False)
    return yaml.safe_dump(data, **kwargs)
