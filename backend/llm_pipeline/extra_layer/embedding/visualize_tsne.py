import pandas as pd
import matplotlib.pyplot as plt
from sklearn.manifold import TSNE

# Load the embeddings
path = "backend/llm_pipeline/data/processed/embeddings.csv"
df = pd.read_csv(path)

# Apply t-SNE
print("🔄 Running t-SNE...")
tsne = TSNE(n_components=2, perplexity=30, random_state=42)
tsne_result = tsne.fit_transform(df)

# Create a DataFrame for plotting
df_tsne = pd.DataFrame(tsne_result, columns=['x', 'y'])

# Plot the results
plt.figure(figsize=(10, 8))
plt.scatter(df_tsne['x'], df_tsne['y'], s=10, alpha=0.7)
plt.title("t-SNE Visualization of Behavior Embeddings")
plt.xlabel("t-SNE Component 1")
plt.ylabel("t-SNE Component 2")
plt.grid(True)
plt.tight_layout()
plt.show()
