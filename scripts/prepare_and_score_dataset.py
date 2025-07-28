#!/usr/bin/env python3
import os
import sys
import pandas as pd
import importlib

# --- Setup project path to import your trust engine ---
SCRIPT_DIR = os.path.dirname(__file__)
PROJECT_ROOT = os.path.abspath(os.path.join(SCRIPT_DIR, '..'))
BACKEND_PATH = os.path.join(PROJECT_ROOT, 'backend')
# Insert backend directory so `app.services` modules are importable
sys.path.insert(0, BACKEND_PATH)

# Dynamically import the trust engine module and retrieve the scoring function
try:
    trust_module = importlib.import_module('app.services.trust_engine_final')
    score_minimal = getattr(trust_module, 'score_minimal')
    if score_minimal is None:
        raise AttributeError
except (ModuleNotFoundError, AttributeError) as e:
    print("Error: Could not import 'score_minimal' from trust_engine_final. Check the module and function name.")
    raise


def prepare_and_score(input_path: str, output_path: str):
    """
    Reads a merged telemetry CSV, fills missing minimal-mode fields,
    computes risk_score via the trust engine, and adds a binary label.
    """
    df = pd.read_csv(input_path)

    # Define minimal-mode expected fields
    minimal_fields = [
        "timestamp", "hostname", "endpoint_id", "os_type",
        "pid", "ppid", "process_name", "cmd", "cpu_percent", "memory"
    ]

    # Fill any missing fields with defaults
    for col in minimal_fields:
        if col not in df.columns:
            df[col] = 0 if col in ("timestamp", "pid", "ppid", "cpu_percent", "memory") else ""

    # Compute risk_score for each row (extract numeric score from tuple)
    df['risk_score'] = df.apply(
        lambda row: score_minimal(row.to_dict())[0],
        axis=1
    )

    # Add label: 1 if risk_score > 0 else 0
    df['label'] = df['risk_score'].apply(lambda x: 1 if x > 0 else 0)

    # Save output CSV
    df.to_csv(output_path, index=False)
    print(f"Prepared and scored dataset: {output_path}")


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description="Score merged telemetry and label for training.")
    parser.add_argument("--input", required=True, help="Path to merged_telemetry.csv")
    parser.add_argument("--output", default="merged_scored_labeled.csv", help="Output CSV path")
    args = parser.parse_args()
    prepare_and_score(args.input, args.output)
