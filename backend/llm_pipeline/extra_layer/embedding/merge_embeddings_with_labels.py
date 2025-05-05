import pandas as pd

def merge_embeddings_with_labels():
    clean_samples_path = "backend_2/llm_pipeline_2/data_2/processed_2/normalized_samples.csv"
    embeddings_path = "backend_2/llm_pipeline_2/data_2/processed_2/embeddings.csv"
    output_path = "backend_2/llm_pipeline_2/data_2/processed_2/labeled_embeddings.csv"

    # Load data
    print("📥 Loading normalized samples...")
    df_clean = pd.read_csv(clean_samples_path)
    print("📥 Loading embeddings...")
    df_embed = pd.read_csv(embeddings_path)

    # Merge on index
    print("🔗 Merging datasets...")
    df_merged = pd.concat([df_embed, df_clean], axis=1)

    # Optional: Add binary labels based on Filename if present
    if "Filename" in df_clean.columns:
        df_merged["label"] = df_clean["Filename"].apply(lambda x: 0 if "Benign" in x else 1)

    # Save
    df_merged.to_csv(output_path, index=False)
    print(f"✅ Merged dataset saved to {output_path}")

if __name__ == "__main__":
    merge_embeddings_with_labels()



