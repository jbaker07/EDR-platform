# backend/llm_pipeline/handlers/llm_handler.py

from transformers import AutoModelForCausalLM, AutoTokenizer
from peft import PeftModel
import torch

# === CONFIG ===
BASE_MODEL_NAME = "mistralai/Mistral-7B-Instruct-v0.1"
ADAPTER_PATH = "backend/llm_pipeline/models/mistral_edr_adapter"



# === MODEL LOADING ===
try:
    print("🔄 Loading base model...")
    base_model = AutoModelForCausalLM.from_pretrained(
        BASE_MODEL_NAME,
        trust_remote_code=True,  # required for custom model architectures
        device_map="auto",       # automatically maps layers to CPU/GPU/MPS
        torch_dtype=torch.float32  # safer for MPS (Mac) or CPU-only environments
    )

    print("🔄 Loading LoRA adapter...")
    model = PeftModel.from_pretrained(
        base_model,
        ADAPTER_PATH
    )

    print("✅ Model and adapter loaded successfully.")

except Exception as e:
    print(f"❌ Failed to load model or adapter: {e}")
    model = None

# === HANDLER FUNCTIONS ===

def get_summary(prompt: str) -> str:
    if model is None:
        return "[Error: Model not loaded]"

    inputs = tokenizer(prompt, return_tensors="pt").to(model.device)
    outputs = model.generate(**inputs, max_new_tokens=100)
    summary = tokenizer.decode(outputs[0], skip_special_tokens=True)
    return summary

def get_hunt_query(threat_name: str) -> str:
    if model is None:
        return "[Error: Model not loaded]"

    prompt = f"Generate a threat hunting query for the threat: {threat_name}"
    inputs = tokenizer(prompt, return_tensors="pt").to(model.device)
    outputs = model.generate(**inputs, max_new_tokens=100)
    hunt_query = tokenizer.decode(outputs[0], skip_special_tokens=True)
    return hunt_query

# === TOKENIZER LOADING (Optional if you need prompt encoding) ===
try:
    tokenizer = AutoTokenizer.from_pretrained(
        BASE_MODEL_NAME,
        trust_remote_code=True
    )
except Exception as e:
    print(f"❌ Failed to load tokenizer: {e}")
    tokenizer = None
