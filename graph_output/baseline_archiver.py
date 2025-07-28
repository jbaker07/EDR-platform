import os
import json
import shutil
import time
from datetime import datetime
from pathlib import Path

ARCHIVE_DIR = Path("baseline_archive")
CURRENT_BASELINE = Path("graph_output/telemetry_graph.gpickle")
METADATA_PATH = ARCHIVE_DIR / "baseline_metadata.json"

def get_timestamp():
    return datetime.utcnow().strftime("%Y%m%d_%H%M%S")

def archive_baseline():
    if not CURRENT_BASELINE.exists():
        print("⚠️ No current graph found to archive.")
        return

    ARCHIVE_DIR.mkdir(parents=True, exist_ok=True)
    timestamp = get_timestamp()
    archive_name = f"graph_snapshot_{timestamp}.gpickle"
    archive_path = ARCHIVE_DIR / archive_name

    shutil.copy(CURRENT_BASELINE, archive_path)

    metadata = {
        "timestamp": timestamp,
        "filename": archive_name,
        "size_bytes": os.path.getsize(archive_path),
        "archived_at": time.time()
    }

    if METADATA_PATH.exists():
        with open(METADATA_PATH, "r") as f:
            existing = json.load(f)
    else:
        existing = []

    existing.append(metadata)

    with open(METADATA_PATH, "w") as f:
        json.dump(existing, f, indent=2)

    print(f"✅ Baseline archived as {archive_name}")

if __name__ == "__main__":
    archive_baseline()
