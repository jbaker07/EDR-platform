import json
from datetime import datetime
from typing import List, Dict, Optional

# Optional: Use OpenAI GPT if needed
USE_OPENAI = False
OPENAI_API_KEY = "sk-..."  # Replace securely in production

try:
    from transformers import pipeline
    summarizer = pipeline("summarization", model="facebook/bart-large-cnn")
except Exception:
    summarizer = None

if USE_OPENAI:
    import openai
    openai.api_key = OPENAI_API_KEY

def summarize_with_openai(prompt: str) -> str:
    try:
        completion = openai.ChatCompletion.create(
            model="gpt-4",
            messages=[{"role": "user", "content": prompt}],
            temperature=0.7,
        )
        return completion['choices'][0]['message']['content']
    except Exception as e:
        return f"[OpenAI Error] {str(e)}"

def summarize_with_local(prompt: str) -> str:
    if summarizer:
        return summarizer(prompt, max_length=180, min_length=60, do_sample=False)[0]['summary_text']
    return "[Local summarizer not available]"

def narrate_replay_chain(replay_events: List[Dict], tags: List[str], shap: Dict, trust: float, role: str = "user") -> str:
    timeline = []
    for event in replay_events:
        ts = event.get("timestamp", "unknown")
        desc = event.get("description", "unknown activity")
        tag = event.get("semantic_tag", "")
        timeline.append(f"• At {ts}, {desc} [{tag}]")

    tags_line = ", ".join(tags)
    shap_top = ", ".join(sorted(shap.keys(), key=shap.get, reverse=True)[:5])

    base_prompt = f"""
    You are a SOC assistant. A session replay has been captured showing potentially malicious behavior. The current trust score is {trust:.2f}. 
    The user role is {role}. Relevant semantic tags include: {tags_line}.
    Key contributing SHAP features include: {shap_top}.
    Here's the timeline:
    {' '.join(timeline)}
    
    Summarize the root cause, behavior pattern, and recommended actions in 1-2 paragraphs.
    """

    return summarize_with_openai(base_prompt) if USE_OPENAI else summarize_with_local(base_prompt)

def explain_root_cause(alert: Dict, session_meta: Optional[Dict] = None) -> str:
    trust_score = alert.get("trust_score", 0.0)
    shap_values = alert.get("shap", {})
    tags = alert.get("tags", [])
    replay = alert.get("replay_events", [])
    role = session_meta.get("role", "user") if session_meta else "user"

    explanation = narrate_replay_chain(replay, tags, shap_values, trust_score, role)

    return json.dumps({
        "summary": explanation,
        "score": trust_score,
        "top_tags": tags,
        "shap_drivers": sorted(shap_values.items(), key=lambda x: x[1], reverse=True)[:5],
        "timestamp": datetime.utcnow().isoformat(),
    }, indent=2)
