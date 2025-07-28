import pandas as pd

benign_path = "backend/gnn_pipeline/data/processed/labeled_telemetry.csv"
malicious_path = "backend/gnn_pipeline/data/processed/full_malicious_injection_pack.csv"
output_path = "backend/gnn_pipeline/data/processed/labeled_telemetry_augmented.csv"

df_benign = pd.read_csv(benign_path)
df_malicious = pd.read_csv(malicious_path)

# Align columns and concatenate
common_cols = df_benign.columns.intersection(df_malicious.columns)
df_merged = pd.concat([df_benign[common_cols], df_malicious[common_cols]], ignore_index=True)

df_merged.to_csv(output_path, index=False)
print(f"[+] Merged dataset saved to: {output_path}")
