import numpy as np
from gensim.models import KeyedVectors
from sklearn.feature_extraction.text import ENGLISH_STOP_WORDS

# Load a pre-trained embedding (update path if needed)
# Example: Download GloVe/Word2Vec in .txt or .bin format
# e.g. GoogleNews-vectors-negative300.bin
embedding_model_path = "models/GoogleNews-vectors-negative300.bin"

try:
    word_vectors = KeyedVectors.load_word2vec_format(embedding_model_path, binary=True)
except Exception as e:
    print("❌ Failed to load embedding model:", e)
    word_vectors = None

def embed_text(text: str) -> np.ndarray:
    """Convert text to an average embedding vector."""
    if not word_vectors:
        return np.zeros(300)

    tokens = [w for w in text.lower().split() if w not in ENGLISH_STOP_WORDS]
    vectors = [word_vectors[word] for word in tokens if word in word_vectors]
    
    if vectors:
        return np.mean(vectors, axis=0)
    else:
        return np.zeros(word_vectors.vector_size)
