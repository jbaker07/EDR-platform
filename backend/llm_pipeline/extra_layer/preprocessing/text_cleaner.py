# backend/llm_pipeline/preprocessing/text_cleaner.py

import re

def clean_text(text: str) -> str:
    """
    Cleans and normalizes raw log/alert text.
    """
    if not text:
        return ""
    
    # Lowercase everything
    text = text.lower()

    # Remove URLs
    text = re.sub(r'http\S+|www\S+', '', text)

    # Remove file paths and IP addresses
    text = re.sub(r'(\/[\w\-\.]+)+', '[FILE_PATH]', text)
    text = re.sub(r'\b(?:\d{1,3}\.){3}\d{1,3}\b', '[IP_ADDRESS]', text)

    # Normalize time strings
    text = re.sub(r'\b\d{1,2}:\d{2}(:\d{2})?\b', '[TIME]', text)

    # Remove special characters and numbers (optional)
    text = re.sub(r'[^a-z\s]', ' ', text)
    text = re.sub(r'\s+', ' ', text).strip()

    return text


def clean_batch(logs: list[str]) -> list[str]:
    """
    Clean a batch of logs or alert strings.
    """
    return [clean_text(log) for log in logs]
