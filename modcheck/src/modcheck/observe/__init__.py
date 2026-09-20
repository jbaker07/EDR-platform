"""Reading what the game actually did.

Everything under this package consumes diagnostics a real runtime emitted. It
produces `observed` evidence -- the only class in the model that is not derived
from an artifact we parsed ourselves.

Two rules hold throughout:

* We do not reconstruct runtime detail. Content Patcher already reports which
  patches loaded, matched their conditions, and applied; we parse its output
  rather than re-deriving those answers, and where its output does not say
  something, neither do we.
* A prediction is only a prediction if it was written down before the
  observation. `prediction.py` exists to make that ordering checkable rather
  than asserted.
"""
from .contentpatcher import (  # noqa: F401
    DumpApplied, DumpOrder, ObservedPatch, Summary,
    parse_dump_applied, parse_dump_order, parse_summary)
from .prediction import (  # noqa: F401
    Claim, Comparison, Prediction, compare, load_predictions)
