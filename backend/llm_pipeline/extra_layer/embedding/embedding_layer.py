# backend/llm_pipeline/extra_layer/embedding/embedding_layer.py

from gensim.models import Word2Vec
from typing import List

def train_word2vec(sentences: List[List[str]], vector_size: int = 100, window: int = 5, min_count: int = 1) -> Word2Vec:
    """
    Trains a Word2Vec model on tokenized sentences.
    """
    model = Word2Vec(sentences=sentences, vector_size=vector_size, window=window, min_count=min_count)
    return model

def get_word_vector(model: Word2Vec, word: str):
    """
    Returns the vector for a word from a trained Word2Vec model.
    """
    if word in model.wv:
        return model.wv[word]
    else:
        return None
