"""Train the DEMO process classifier used by ``POST /api/predict``.

============================  HONESTY NOTICE  ============================
This is a *DEMO* model trained on a TRANSPARENT SYNTHETIC dataset. It is
NOT a real-incident detector and carries no security guarantees. Its only
job is to make ``POST /api/predict`` return a real, deterministic
prediction out-of-the-box on a fresh clone, so the serving path can be
exercised end-to-end without shipping a large pretrained binary.

The synthetic generator encodes one obvious, human-readable rule of thumb:
    high CPU + high memory  =>  "malicious"
    low  CPU + low  memory  =>  "benign"
PID is included only to MATCH THE FEATURE CONTRACT (see below); it carries
no real signal and the generator deliberately makes it label-independent.
=========================================================================

Feature contract (MUST match backend/main.py ``/api/predict``)
--------------------------------------------------------------
``POST /api/predict`` accepts ``{"features": [<float>, ...]}`` and feeds the
list straight into ``model.predict([features])``. The historical labeled
telemetry CSV (backend/telemetry_logs/labeled_telemetry.csv) and the legacy
``train_from_real.py`` both used this exact column order, which we preserve:

    index 0 -> cpu_percent   (percent, ~0..100)
    index 1 -> memory        (bytes; large integers, hence the StandardScaler)
    index 2 -> pid           (process id; contract filler, no real signal)

So a client sends e.g. ``{"features": [3.2, 50000000, 412]}``.

Output: a scikit-learn Pipeline(StandardScaler + LogisticRegression),
classes_ == [0, 1] (0 = benign, 1 = malicious), saved to
``backend/ml/models/demo_classifier.joblib`` (kept tiny, < 200 KB).
"""

from __future__ import annotations

from pathlib import Path

import numpy as np
from joblib import dump
from sklearn.linear_model import LogisticRegression
from sklearn.pipeline import Pipeline
from sklearn.preprocessing import StandardScaler

# The ONE source of truth for feature order. Imported by the serving layer
# (backend/main.py) so the vector fed to .predict() always lines up with how
# the model was trained.
FEATURE_ORDER = ["cpu_percent", "memory", "pid"]

MODEL_DIR = Path(__file__).resolve().parent / "models"
MODEL_PATH = MODEL_DIR / "demo_classifier.joblib"


def make_synthetic_dataset(n: int = 4000, seed: int = 42):
    """Generate a transparent synthetic [cpu_percent, memory, pid] dataset.

    Rule (deliberately simple and human-readable):
      * benign    processes: low CPU (~0-15%) and low memory (~10-300 MB)
      * malicious processes: high CPU (~60-100%) and high memory (~1-4 GB)
    A little Gaussian noise keeps the two clusters from being trivially
    separable. ``pid`` is drawn label-independently on purpose so the model
    cannot (and must not) lean on it.
    """
    rng = np.random.default_rng(seed)
    half = n // 2

    # --- benign cluster ---
    benign_cpu = np.clip(rng.normal(5.0, 4.0, half), 0.0, 100.0)
    benign_mem = np.clip(rng.normal(120e6, 60e6, half), 1e6, None)  # ~120 MB

    # --- malicious cluster ---
    mal_cpu = np.clip(rng.normal(82.0, 10.0, half), 0.0, 100.0)
    mal_mem = np.clip(rng.normal(2.4e9, 0.7e9, half), 1e6, None)  # ~2.4 GB

    cpu = np.concatenate([benign_cpu, mal_cpu])
    mem = np.concatenate([benign_mem, mal_mem])
    # Label-independent PID, same range for both classes (no real signal).
    pid = rng.integers(1, 60000, size=n).astype(float)

    X = np.column_stack([cpu, mem, pid])  # order == FEATURE_ORDER
    y = np.concatenate([np.zeros(half, dtype=int), np.ones(half, dtype=int)])

    # Shuffle so the train/test split (and the fit) doesn't see ordered classes.
    perm = rng.permutation(n)
    return X[perm], y[perm]


def train() -> Pipeline:
    X, y = make_synthetic_dataset()
    pipeline = Pipeline(
        [
            ("scaler", StandardScaler()),
            # Tiny, deterministic, and serializes to a few KB.
            ("clf", LogisticRegression(max_iter=1000)),
        ]
    )
    pipeline.fit(X, y)

    MODEL_DIR.mkdir(parents=True, exist_ok=True)
    dump(pipeline, MODEL_PATH)
    return pipeline


def main() -> None:
    pipeline = train()
    acc = pipeline.score(*make_synthetic_dataset(n=1000, seed=7))
    size_kb = MODEL_PATH.stat().st_size / 1024
    print(f"Feature order: {FEATURE_ORDER}")
    print(f"Saved DEMO model -> {MODEL_PATH} ({size_kb:.1f} KB)")
    print(f"Held-out synthetic accuracy: {acc:.3f}")
    print("NOTE: synthetic demo model, NOT a real-incident detector.")


if __name__ == "__main__":
    main()
