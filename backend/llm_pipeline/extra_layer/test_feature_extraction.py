from preprocessing.text_cleaner import clean_text
from feature_extraction.extractor import extract_features
from feature_extraction.vectorizer import vectorize_text_bow, vectorize_text_tfidf
from embedding.embedder import embed_text
from extra_layer.data_collection.load_alerts import load_alerts


if __name__ == "__main__":
    alerts = load_alerts("llm_pipeline/extra_layer/data_collection/sample_alerts.json")
    for alert in alerts[:5]:  # Only process a few to start
        for proc in alert["processes"]:
            text = f"{proc['name']} {proc['cmd']}"
        ...
        # run your full pipeline: clean_text -> extract_features -> vectorize_text_bow/tfidf -> embed

    cleaned = clean_text(text)
    print("Original:", text)
    print("Cleaned:", cleaned)

    features = extract_features(cleaned)
    print("Extracted Features:", features)

    bow_vector, bow_features = vectorize_text_bow([cleaned])
    print("BoW Vector:", bow_vector)
    print("BoW Features:", bow_features)

    tfidf_vector, tfidf_features = vectorize_text_tfidf([cleaned])
    print("TF-IDF Vector:", tfidf_vector)
    print("TF-IDF Features:", tfidf_features)

    embedding = embed_text(cleaned)
    print("Embedding Vector (truncated):", embedding[:10])  # Just show first 10 numbers

