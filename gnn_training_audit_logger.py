import hashlib
import json
import os
import sys
import time
from datetime import datetime
from typing import Dict, Optional

AUDIT_LOG_PATH = "gnn_pipeline/audits/training_audit_log.jsonl"


def hash_file(filepath: str) -> str:
    """Return SHA-256 hash of a file."""
    sha256 = hashlib.sha256()
    with open(filepath, "rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            sha256.update(chunk)
    return sha256.hexdigest()


def generate_audit_record(
    dataset_path: str,
    model_output_path: str,
    training_params: Dict,
    tag_summary: Optional[Dict] = None,
    anomaly_notes: Optional[str] = None
) -> Dict:
    """Create structured audit log entry."""
    timestamp = datetime.utcnow().isoformat()
    dataset_hash = hash_file(dataset_path)
    model_checksum = hash_file(model_output_path)

    return {
        "timestamp": timestamp,
        "dataset": {
            "path": dataset_path,
            "sha256": dataset_hash
        },
        "model_output": {
            "path": model_output_path,
            "sha256": model_checksum
        },
        "training_parameters": training_params,
        "tag_summary": tag_summary or {},
        "anomalies": anomaly_notes or "",
    }


def append_audit_record(audit_record: Dict, log_path: str = AUDIT_LOG_PATH):
    os.makedirs(os.path.dirname(log_path), exist_ok=True)
    with open(log_path, "a") as log_file:
        log_file.write(json.dumps(audit_record) + "\n")


def audit_training_run(
    dataset_path: str,
    model_output_path: str,
    training_params: Dict,
    tag_summary: Optional[Dict] = None,
    anomaly_notes: Optional[str] = None
):
    """Main callable function to perform audit."""
    record = generate_audit_record(
        dataset_path, model_output_path, training_params, tag_summary, anomaly_notes
    )
    append_audit_record(record)


# Optional CLI support
if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("Usage: python gnn_training_audit_logger.py <dataset_path> <model_output_path> <training_params_json>")
        sys.exit(1)

    dataset_path = sys.argv[1]
    model_output_path = sys.argv[2]
    training_params = json.loads(sys.argv[3])

    audit_training_run(dataset_path, model_output_path, training_params)
    print(f"✅ Audit record written for model output: {model_output_path}")
