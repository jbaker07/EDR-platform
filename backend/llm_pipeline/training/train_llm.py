# backend/llm_pipeline/training/train_llm.py

import pandas as pd
import torch
from sklearn.model_selection import train_test_split
from torch.utils.data import DataLoader, TensorDataset
from torch import nn
from transformers import Trainer, TrainingArguments

CSV_PATH = "llm_pipeline/data/processed/clean_samples.csv"

print("🟡 Loading CSV SAMPLE...")
try:
    df = pd.read_csv(CSV_PATH, engine="python", on_bad_lines="skip")
    print(f"✅ Loaded CSV with shape: {df.shape}")
except Exception as e:
    print(f"❌ Failed to load CSV: {e}")
    exit(1)

print("🧹 Columns:", list(df.columns))

# Separate features and labels
X = df.drop(columns=["label"]).values.astype("float32")  # <-- lowercase 'label'
y = df["label"].values.astype("int64")

print(f"📦 Features shape: {X.shape} | Labels shape: {y.shape}")

# Split
X_train, X_val, y_train, y_val = train_test_split(X, y, test_size=0.2, random_state=42)

# Convert to PyTorch tensors
train_dataset = TensorDataset(torch.tensor(X_train), torch.tensor(y_train))
val_dataset = TensorDataset(torch.tensor(X_val), torch.tensor(y_val))

# Define simple MLP
class MLPClassifier(nn.Module):
    def __init__(self, input_dim):
        super().__init__()
        self.layers = nn.Sequential(
            nn.Linear(input_dim, 512),
            nn.ReLU(),
            nn.Dropout(0.2),
            nn.Linear(512, 128),
            nn.ReLU(),
            nn.Linear(128, 2)
        )

    def forward(self, x):
        return self.layers(x)

model = MLPClassifier(X.shape[1])

# Training setup
training_args = TrainingArguments(
    output_dir="./results",
    num_train_epochs=3,
    per_device_train_batch_size=32,
    per_device_eval_batch_size=64,
    evaluation_strategy="epoch",
    save_strategy="epoch",
    logging_dir="./logs",
    logging_steps=50,
    report_to="none",  # disable wandb/huggingface logging unless you want it
)

# Custom Trainer
def compute_metrics(eval_pred):
    logits, labels = eval_pred
    preds = torch.argmax(torch.tensor(logits), dim=1)
    acc = (preds == torch.tensor(labels)).float().mean()
    return {"accuracy": acc.item()}

trainer = Trainer(
    model=model,
    args=training_args,
    train_dataset=train_dataset,
    eval_dataset=val_dataset,
    compute_metrics=compute_metrics,
)

if __name__ == "__main__":
    trainer.train()
