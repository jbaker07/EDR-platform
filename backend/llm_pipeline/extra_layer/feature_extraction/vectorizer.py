from sklearn.feature_extraction.text import TfidfVectorizer
import numpy as np
from sklearn.feature_extraction.text import CountVectorizer, TfidfVectorizer

def vectorize_text_bow(texts, max_features=1000):
    """
    Convert a list of texts into Bag-of-Words vectors.
    Returns the vector array and the feature names.
    """
    vectorizer = CountVectorizer(max_features=max_features)
    vectors = vectorizer.fit_transform(texts)
    return vectors.toarray(), vectorizer.get_feature_names_out()

def vectorize_text_tfidf(texts, max_features=1000):
    """
    Convert a list of texts into TF-IDF vectors.
    Returns the vector array and the feature names.
    """
    vectorizer = TfidfVectorizer(max_features=max_features)
    vectors = vectorizer.fit_transform(texts)
    return vectors.toarray(), vectorizer.get_feature_names_out()


class TextVectorizer:
    def __init__(self, max_features: int = 1000):
        self.vectorizer = TfidfVectorizer(max_features=max_features)

    def fit(self, texts: list[str]):
        self.vectorizer.fit(texts)

    def transform(self, texts: list[str]) -> np.ndarray:
        return self.vectorizer.transform(texts).toarray()

    def fit_transform(self, texts: list[str]) -> np.ndarray:
        return self.vectorizer.fit_transform(texts).toarray()

    def get_feature_names(self):
        return self.vectorizer.get_feature_names_out()
