"""JSON as the Stardew Valley ecosystem actually writes it.

SMAPI content packs are JSON with comments and trailing commas. A real
published pack -- MouseyPounds' Bear Mounts -- has a trailing comma inside its
ConfigSchema and inside its first patch, and strict ``json.loads`` rejects it
outright.

That mattered more than it sounds: an inspector that cannot read a pack's
content.json reports it as having no patches at all, which is a confidently
clean result for a mod full of patches. Reading it as strict JSON is not a
conservative choice, it is a wrong answer.

The stripper walks the text rather than pattern-matching, because a regex eats
the rest of any line containing a URL. It was written for SMAPI's own metadata
file and is shared here rather than duplicated.
"""
from __future__ import annotations

import re

_TRAILING_COMMA = re.compile(r",(\s*[}\]])")


def strip_jsonc(raw: bytes | str) -> str:
    """Remove // and /* */ comments and trailing commas, respecting strings."""
    text = raw.decode("utf-8-sig") if isinstance(raw, bytes) else raw
    out: list[str] = []
    i, n, in_string = 0, len(text), False
    while i < n:
        char = text[i]
        if in_string:
            out.append(char)
            if char == "\\" and i + 1 < n:
                out.append(text[i + 1])
                i += 2
                continue
            if char == '"':
                in_string = False
            i += 1
            continue
        if char == '"':
            in_string = True
            out.append(char)
            i += 1
            continue
        if text.startswith("/*", i):
            end = text.find("*/", i + 2)
            i = end + 2 if end >= 0 else n
            continue
        if text.startswith("//", i):
            end = text.find("\n", i)
            i = end if end >= 0 else n
            continue
        out.append(char)
        i += 1
    return _TRAILING_COMMA.sub(r"\1", "".join(out))
