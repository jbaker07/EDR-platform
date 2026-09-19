"""Artifact inspection.

Inspectors read real mod artifacts and report what is actually inside them.
Every fact they emit carries an evidence class:

* ``declared``  — the artifact's own manifest says so (an author's claim).
* ``extracted`` — read mechanically out of the bytes.

They never emit ``derived`` facts; that is the analysis layer's job. Each
inspector also reports its *coverage*: what it looked at and what it did not,
so a clean inspection is never mistaken for a guarantee.
"""
from .base import Artifact, Fact, Inspection, InspectionError  # noqa: F401
from .registry import inspect_path, detect_kind, INSPECTORS  # noqa: F401
