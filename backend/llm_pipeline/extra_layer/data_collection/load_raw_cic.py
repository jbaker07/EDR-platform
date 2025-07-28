import pandas as pd
import os

def load_and_merge_csvs(folder_path: str) -> pd.DataFrame:
    all_files = [os.path.join(folder_path, f) for f in os.listdir(folder_path) if f.endswith('.csv')]
    df_list = [pd.read_csv(file) for file in all_files]
    merged_df = pd.concat(df_list, ignore_index=True)
    return merged_df

if __name__ == "__main__":
    data_folder = "backend/llm_pipeline/data/raw"
    df = load_and_merge_csvs(data_folder)
    print(f"✅ Loaded {len(df)} records with {len(df.columns)} columns.")
    df.to_csv("backend/llm_pipeline/data/clean/merged_dynamic.csv", index=False)
    print("✅ Saved merged CSV to clean folder.")
