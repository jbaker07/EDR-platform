import pandas as pd
import numpy as np

def load_clean_data(path="llm_pipeline/data/processed/clean_samples.csv") -> pd.DataFrame:
    df = pd.read_csv(path)
    print(f"✅ Loaded {len(df)} rows and {len(df.columns)} columns.")
    return df

def normalize_features(df: pd.DataFrame) -> pd.DataFrame:
    # Drop non-numeric or identifier columns
    non_features = ['Filename']
    df_numeric = df.drop(columns=non_features)
    
    # Fill missing values
    df_filled = df_numeric.fillna(0)
    
    # Normalize to 0-1
    df_normalized = (df_filled - df_filled.min()) / (df_filled.max() - df_filled.min())
    
    # Concatenate Filename back
    df_final = pd.concat([df['Filename'], df_normalized], axis=1)
    return df_final

def save_normalized_data(df: pd.DataFrame, output_path="backend/llm_pipeline/data/processed/normalized_samples.csv"):
    df.to_csv(output_path, index=False)
    print(f"✅ Saved normalized data to {output_path}")

if __name__ == "__main__":
    df = load_clean_data()
    df_normalized = normalize_features(df)
    save_normalized_data(df_normalized)
