import os
import requests
from typing import Dict

# You can change this to your actual endpoint, e.g., "http://localhost:11434/api/generate" for Ollama
LLM_ENDPOINT = os.getenv("LOCAL_LLM_ENDPOINT", "http://localhost:11434/api/generate")
LLM_MODEL = os.getenv("LOCAL_LLM_MODEL", "llama3")

HEADERS = {
    "Content-Type": "application/json"
}


def query_llm(prompt: str, system_prompt: str = "", temperature: float = 0.4, max_tokens: int = 1024) -> str:
    payload = {
        "model": LLM_MODEL,
        "prompt": prompt,
        "system": system_prompt,
        "stream": False,
        "temperature": temperature,
        "max_tokens": max_tokens
    }

    try:
        response = requests.post(LLM_ENDPOINT, json=payload, headers=HEADERS, timeout=30)
        response.raise_for_status()
        data = response.json()
        return data.get("response", "").strip()
    except requests.exceptions.RequestException as e:
        return f"[LLM ERROR] {str(e)}"
