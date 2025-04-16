import re
from typing import List

class TextCleaner:
    def __init__(self):
        # Define common patterns to remove
        self.patterns = [
            (r"http\S+", ""),              # remove URLs
            (r"\S+@\S+", ""),              # remove emails
            (r"\d{1,2}[:.]\d{2}", ""),     # remove timestamps like 14:30 or 12.45
            (r"\b(?:pid|uid|exe)\b[:=]?\d*", ""),  # remove common process metadata
            (r"[^a-zA-Z0-9\s]", " "),      # remove special characters
            (r"\s+", " "),                 # collapse whitespace
        ]
        self.stopwords = {"the", "a", "an", "is", "it", "in", "at", "to", "on", "of", "for"}

    def clean(self, text: str) -> str:
        text = text.lower()
        for pattern, replacement in self.patterns:
            text = re.sub(pattern, replacement, text)
        return text.strip()

    def clean_tokens(self, text: str) -> List[str]:
        cleaned = self.clean(text)
        return [token for token in cleaned.split() if token not in self.stopwords]
