from transformers import AutoModelForCausalLM, AutoTokenizer, TrainingArguments, Trainer
from peft import get_peft_model, LoraConfig, TaskType
from datasets import load_dataset
import torch

model_id = "mistralai/Mistral-7B-Instruct-v0.1"
tokenizer = AutoTokenizer.from_pretrained(model_id, trust_remote_code=True)
tokenizer.pad_token = tokenizer.eos_token  # ✅ ADD THIS LINE

model = AutoModelForCausalLM.from_pretrained(
    model_id,
    torch_dtype=torch.float16,
    device_map="auto"
)


lora_config = LoraConfig(
    r=16,
    lora_alpha=32,
    lora_dropout=0.05,
    bias="none",
    task_type=TaskType.CAUSAL_LM,
    target_modules=["q_proj", "v_proj"]
)

model = get_peft_model(model, lora_config)
model = model.to("cpu")  # ✅ Force CPU


dataset = load_dataset("json", data_files="llm_pipeline/data/red_alerts.jsonl")


def preprocess(example):
    prompt = f"""### Instruction:\n{example['instruction']}\n\n### Input:\n{example['input']}\n\n### Response:\n{example['output']}"""
    encoding = tokenizer(prompt, truncation=True, padding="max_length", max_length=1024)
    encoding["labels"] = encoding["input_ids"].copy()  # ✅ Set labels
    return encoding


tokenized = dataset["train"].map(preprocess)

training_args = TrainingArguments(
    output_dir="llm_pipeline/models/mistral_edr_adapter",
    per_device_train_batch_size=4,
    gradient_accumulation_steps=4,
    num_train_epochs=5,
    learning_rate=2e-4,
    warmup_steps=50,
    fp16=False,
    no_cuda=True,  # ✅ Force CPU training
    logging_steps=10,
    save_strategy="epoch",
    report_to="none"
)


trainer = Trainer(
    model=model,
    args=training_args,
    train_dataset=tokenized,
    tokenizer=tokenizer
)

trainer.train()
