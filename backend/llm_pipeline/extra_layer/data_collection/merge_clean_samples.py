import pandas as pd
import os

RAW_DIR = "llm_pipeline/data/raw"
PROCESSED_DIR = "llm_pipeline/data/processed"
OUTPUT_FILE = os.path.join(PROCESSED_DIR, "clean_samples.csv")

def load_and_merge_csvs(file_list):
    dfs = []
    for file in file_list:
        path = os.path.join(RAW_DIR, file)
        print(f"Loading {path}")
        df = pd.read_csv(path)
        dfs.append(df)
    combined = pd.concat(dfs, ignore_index=True)
    return combined

def clean_data(df):
    df = df.dropna()
    df = df.drop_duplicates()
    return df

def save_clean_samples():
    files = ["output1.csv", "output2.csv", "output3.csv"]
    merged_df = load_and_merge_csvs(files)
    clean_df = clean_data(merged_df)
    os.makedirs(PROCESSED_DIR, exist_ok=True)
    clean_df.to_csv(OUTPUT_FILE, index=False)
    print(f"✅ Clean dataset saved to: {OUTPUT_FILE}")
    print(f"🧼 Final shape: {clean_df.shape}")

if __name__ == "__main__":
    save_clean_samples()
