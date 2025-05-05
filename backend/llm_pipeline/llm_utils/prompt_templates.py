def build_prompt(instruction: str, alert: dict) -> str:
    return f"### Instruction:\n{instruction}\n\n### Input:\n{alert}\n\n### Response:\n"
