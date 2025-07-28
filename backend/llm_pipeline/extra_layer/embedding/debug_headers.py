# File: llm_pipeline/extra_layer/embedding/debug_headers.py

import pandas as pd

df = pd.read_csv("backend/llm_pipeline/data/processed/embeddings.csv")
print("🧾 Columns in embeddings.csv:")
print(df.columns.tolist())
