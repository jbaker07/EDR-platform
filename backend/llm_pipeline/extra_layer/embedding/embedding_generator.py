import pandas as pd
from sklearn.decomposition import PCA
from sklearn.preprocessing import StandardScaler
from sklearn.impute import SimpleImputer
import os

def load_data(path: str):
    print(f"📥 Loading data from {path}")
    return pd.read_csv(path)

def generate_embeddings(df: pd.DataFrame, n_components: int = 10):
    print("🔎 Preprocessing for PCA...")
    features = df.drop(columns=['Filename'], errors='ignore')

    # ✅ Impute missing values with column means
    imputer = SimpleImputer(strategy="mean")
    features_imputed = imputer.fit_transform(features)

    # ✅ Normalize data
    scaler = StandardScaler()
    features_scaled = scaler.fit_transform(features_imputed)

    # ✅ PCA embedding
    pca = PCA(n_components=n_components)
    embeddings = pca.fit_transform(features_scaled)

    print(f"✅ Generated embeddings with shape: {embeddings.shape}")
    return pd.DataFrame(embeddings, columns=[f"pc_{i+1}" for i in range(n_components)])

def save_embeddings(df_embeddings: pd.DataFrame, output_path: str):
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    df_embeddings.to_csv(output_path, index=False)
    print(f"💾 Embeddings saved to {output_path}")

if __name__ == "__main__":
    input_path = "backend/llm_pipeline/data/processed/normalized_samples.csv"
    output_path = "backend/llm_pipeline/data/processed/embeddings.csv"

    df = load_data(input_path)
    df_embeddings = generate_embeddings(df)
    save_embeddings(df_embeddings, output_path)

