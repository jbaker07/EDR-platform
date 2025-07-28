from extra_layer.preprocessing.text_cleaner import clean_text


def extract_features(text: str) -> dict:
    cleaned = clean_text(text)
    return {
        "word_count": len(cleaned.split()),
        "char_count": len(cleaned),
        # Add more features here
    }
