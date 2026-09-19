"""Version comparison for the ecosystems we resolve dependencies in.

Two different systems, implemented to their own specifications rather than
approximated with one generic parser:

* **Fabric** uses a Semantic Versioning 2.0.0 *superset* allowing an arbitrary
  number of components, an empty pre-release and arbitrary build metadata.
  Space-separated predicates within one string are ANDed; an array of strings is
  ORed. Versions that do not fit the superset still compare for equality.
* **Forge / NeoForge** use Maven version ranges (``[1.21,1.22)``, ``[47,)``).

Anything we cannot decide returns ``None`` rather than a guess, so an
undecidable constraint is reported as unresolved instead of as a failure.
"""
from __future__ import annotations

import re
from dataclasses import dataclass
from typing import Iterable

_SEMVER = re.compile(r"""
    ^(?P<core>\d+(?:\.\d+)*)
     (?:-(?P<pre>[0-9A-Za-z.\-]*))?
     (?:\+(?P<build>[0-9A-Za-z.\-]+))?$
""", re.VERBOSE)

_PREDICATE = re.compile(r"^(?P<op>>=|<=|!=|>|<|\^|~|=)?\s*(?P<version>.+)$")


@dataclass(frozen=True)
class Version:
    """A Fabric-superset semantic version.

    ``pre`` is ``None`` for no pre-release, ``""`` for the empty pre-release
    (the earliest possible one, which is what ``1.2-`` means). Build metadata is
    parsed and then ignored for comparison, as the specification requires.
    """

    components: tuple[int, ...]
    pre: str | None = None
    raw: str = ""
    parsed: bool = True

    @classmethod
    def parse(cls, text: str) -> "Version":
        text = (text or "").strip()
        match = _SEMVER.match(text)
        if not match:
            return cls(components=(), pre=None, raw=text, parsed=False)
        core = tuple(int(part) for part in match.group("core").split("."))
        return cls(components=core, pre=match.group("pre"), raw=text)

    def component(self, index: int) -> int:
        """Absent components compare as 0."""
        return self.components[index] if index < len(self.components) else 0

    def _pre_key(self) -> tuple:
        # No pre-release outranks any pre-release; the empty pre-release is the
        # earliest of them.
        if self.pre is None:
            return (2,)
        if self.pre == "":
            return (0,)
        parts = []
        for ident in self.pre.split("."):
            if ident.isdigit():
                parts.append((0, int(ident), ""))
            else:
                parts.append((1, 0, ident))
        return (1, tuple(parts))

    def compare(self, other: "Version") -> int | None:
        if not self.parsed or not other.parsed:
            # Comparison support is limited to equality for non-conforming versions.
            return 0 if self.raw == other.raw else None
        width = max(len(self.components), len(other.components))
        for i in range(width):
            a, b = self.component(i), other.component(i)
            if a != b:
                return -1 if a < b else 1
        a_key, b_key = self._pre_key(), other._pre_key()
        if a_key == b_key:
            return 0
        return -1 if a_key < b_key else 1

    def __str__(self) -> str:
        return self.raw


def _bump(version: Version, index: int) -> Version:
    """Version with component `index` incremented and the rest zeroed, empty pre."""
    components = [version.component(i) for i in range(max(index + 1, len(version.components)))]
    components[index] += 1
    for i in range(index + 1, len(components)):
        components[i] = 0
    return Version(components=tuple(components[:index + 1] + [0] * (2 - index)), pre="",
                   raw="derived")


def _satisfies_predicate(version: Version, predicate: str) -> bool | None:
    predicate = predicate.strip()
    if not predicate or predicate == "*":
        return True

    # X-ranges: 1.x, 1.2.X, 1.*
    x_match = re.match(r"^(\d+(?:\.\d+)*)\.(?:x|X|\*)(?:\.(?:x|X|\*))*$", predicate)
    if x_match:
        base = Version.parse(x_match.group(1))
        lower = Version(components=base.components, pre="", raw=predicate)
        upper = _bump(base, len(base.components) - 1)
        low = version.compare(lower)
        high = version.compare(upper)
        if low is None or high is None:
            return None
        return low >= 0 and high < 0

    match = _PREDICATE.match(predicate)
    if not match:
        return None
    op = match.group("op") or "="
    target = Version.parse(match.group("version"))

    if op in ("^", "~"):
        if not target.parsed:
            return None
        index = 0 if op == "^" else 1
        lower = target
        upper = _bump(target, index)
        low = version.compare(lower)
        high = version.compare(upper)
        if low is None or high is None:
            return None
        return low >= 0 and high < 0

    result = version.compare(target)
    if result is None:
        return None
    return {
        "=": result == 0,
        "!=": result != 0,
        ">": result > 0,
        ">=": result >= 0,
        "<": result < 0,
        "<=": result <= 0,
    }[op]


def fabric_satisfies(version_text: str, constraint: str | Iterable[str]) -> bool | None:
    """Does `version_text` satisfy a fabric.mod.json version range?

    Returns None when the constraint or version cannot be decided.
    """
    if isinstance(constraint, str):
        alternatives = [constraint]
    else:
        alternatives = list(constraint)
    if not alternatives:
        return True

    version = Version.parse(version_text)
    any_unknown = False
    for alternative in alternatives:  # array elements are ORed
        if alternative is None:
            continue
        results = [_satisfies_predicate(version, part)
                   for part in str(alternative).split() if part]
        if not results:
            return True
        if all(r is True for r in results):  # space-separated parts are ANDed
            return True
        if any(r is None for r in results) and not any(r is False for r in results):
            any_unknown = True
    return None if any_unknown else False


_MAVEN_RANGE = re.compile(r"^\s*([\[\(])\s*([^,\]\)]*)\s*,\s*([^,\]\)]*)\s*([\]\)])\s*$")


def maven_satisfies(version_text: str, constraint: str) -> bool | None:
    """Does `version_text` satisfy a Maven version range, as Forge/NeoForge use?

    A bare version in Maven means "recommended, but any version allowed", so it
    is not a hard constraint and returns True.
    """
    constraint = (constraint or "").strip()
    if not constraint or constraint == "*":
        return True

    # A union of ranges, e.g. "[1.0,2.0),[3.0,)"
    parts = re.findall(r"[\[\(][^\[\]\(\)]*[\]\)]", constraint)
    if parts:
        unknown = False
        for part in parts:
            result = _maven_single(version_text, part)
            if result is True:
                return True
            if result is None:
                unknown = True
        return None if unknown else False
    return True  # bare version: a soft recommendation, not a requirement


def _maven_single(version_text: str, constraint: str) -> bool | None:
    match = _MAVEN_RANGE.match(constraint)
    if not match:
        return None
    open_b, low_text, high_text, close_b = match.groups()
    version = Version.parse(version_text)

    if low_text:
        result = version.compare(Version.parse(low_text))
        if result is None:
            return None
        if open_b == "[" and result < 0:
            return False
        if open_b == "(" and result <= 0:
            return False
    if high_text:
        result = version.compare(Version.parse(high_text))
        if result is None:
            return None
        if close_b == "]" and result > 0:
            return False
        if close_b == ")" and result >= 0:
            return False
    return True


SATISFIERS = {
    "fabric": fabric_satisfies,
    "quilt": fabric_satisfies,
    "smapi": fabric_satisfies,
    "forge": maven_satisfies,
    "neoforge": maven_satisfies,
}


def satisfies(loader: str, version_text: str, constraint) -> bool | None:
    """Dispatch to the constraint syntax the loader actually uses."""
    fn = SATISFIERS.get(loader)
    if fn is None:
        return None
    return fn(version_text, constraint)
