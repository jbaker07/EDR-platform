# llm_pipeline/training/prepare_data.py

import pandas as pd
import os

PARQUET_INPUT = "ember/archive/train_ember_2018_v2_features.parquet"
CSV_OUTPUT = "llm_pipeline/data/processed/clean_samples.csv"

def main():
    print(f"📥 Loading: {PARQUET_INPUT}")
    df = pd.read_parquet(PARQUET_INPUT)
    print(f"Initial shape: {df.shape}")

    # Drop rows with NA values
    df = df.dropna()
    print(f"After dropping NAs: {df.shape}")

    # Ensure the Label column exists
    if "Label" not in df.columns:
        raise ValueError("❌ No 'Label' column found in the data.")

    # Filter out rows where Label is not 0 or 1 (optional)
    df = df[df["Label"].isin([0, 1])]
    print(f"After filtering labels [0, 1]: {df.shape}")

    # Add a 'label' column for clarity if needed
    df = df.rename(columns={"Label": "label"})

    # Create output directory if it doesn't exist
    os.makedirs(os.path.dirname(CSV_OUTPUT), exist_ok=True)

    # Save to CSV
    df.to_csv(CSV_OUTPUT, index=False)
    print(f"✅ Saved cleaned dataset with {len(df)} samples to: {CSV_OUTPUT}")

if __name__ == "__main__":
    main()
