import pandas as pd
from pathlib import Path

# Input/output paths
INPUT_PATH = Path("backend/telemetry_logs/real_telemetry_log.csv")
OUTPUT_PATH = Path("backend/telemetry_logs/labeled_telemetry.csv")

# Load telemetry
df = pd.read_csv(INPUT_PATH)

# Define labeling rules (basic heuristic-based for now)
def label_row(row):
    suspicious_cmds = ["/tmp", "malware", ".sh", ".py"]
    if (
        row.get("cpu_percent", 0) > 90 or
        any(keyword in str(row.get("cmd", "")).lower() for keyword in suspicious_cmds)
    ):
        return "malicious"
    return "benign"

# Apply label
df["label"] = df.apply(label_row, axis=1)

# Save labeled data
df.to_csv(OUTPUT_PATH, index=False)
print(f"✅ Labeled {len(df)} telemetry entries → {OUTPUT_PATH}")
