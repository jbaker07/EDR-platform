# File: llm_pipeline/extra_layer/embedding/add_labels.py

import pandas as pd

def add_labels(path="backend/llm_pipeline/data/processed/embeddings.csv"):
    df = pd.read_csv(path)

    # Label: 1 for malware, 0 for benign
    df["label"] = df["Filename"].apply(lambda x: 0 if "Benign" in x else 1)

    df.to_csv(path, index=False)
    print(f"✅ Labels added and saved to {path}")

if __name__ == "__main__":
    add_labels()
