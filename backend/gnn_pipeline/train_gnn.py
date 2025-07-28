import torch
import torch.optim as optim
import torch.nn as nn
from sklearn.preprocessing import StandardScaler, OneHotEncoder
from sklearn.model_selection import train_test_split
import pandas as pd
import numpy as np
from torch_geometric.data import Data
from gnn_pipeline.model import GNNModel  # Ensure this matches your model definition

# --- 1. Load and preprocess the dataset ---
df = pd.read_csv("data/merged_scored_labeled.csv")

# --- 1.1 Feature Selection ---
# Numerical features
numerical_columns = ['cpu_percent', 'memory', 'risk_score']
X_numerical = df[numerical_columns].values

# Categorical features (one-hot encode)
categorical_columns = ['process_name', 'cmd', 'os_type']
X_categorical = df[categorical_columns]

# --- 1.2 Scaling Numerical Features ---
scaler = StandardScaler()
X_numerical = scaler.fit_transform(X_numerical)

# --- 1.3 One-Hot Encoding Categorical Features ---
encoder = OneHotEncoder(sparse_output=False)  # Fix for the new version of scikit-learn
X_categorical_encoded = encoder.fit_transform(X_categorical)

# --- 1.4 Combine the Features ---
X = np.hstack([X_numerical, X_categorical_encoded])

# --- 1.5 Target Labels ---
y = df['label'].values

# --- 2. Split Data into Train and Test ---
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

# Convert to torch tensors
X_train = torch.tensor(X_train, dtype=torch.float32)
y_train = torch.tensor(y_train, dtype=torch.long)
X_test = torch.tensor(X_test, dtype=torch.float32)
y_test = torch.tensor(y_test, dtype=torch.long)

# --- 3. Create the Graph Data Object ---
# Assume a simple fully connected graph for now (you can replace this with your actual edge creation logic)
edges = []
for i in range(len(X_train)):
    for j in range(i + 1, len(X_train)):
        edges.append([i, j])
        edges.append([j, i])  # Undirected graph

edge_index = torch.tensor(edges, dtype=torch.long).t().contiguous()

# Create the graph data object for PyTorch Geometric
train_data = Data(x=X_train, edge_index=edge_index, y=y_train)

# --- 4. Initialize the Model, Loss, and Optimizer ---
input_dim = X_train.shape[1]  # Number of features (10 after combining)
hidden_channels = 64
output_channels = len(np.unique(y))  # Number of classes (labels)

# Initialize the model
model = GNNModel(in_channels=input_dim, hidden_channels=hidden_channels, out_channels=output_channels)

# Loss and Optimizer
# Apply class weights to penalize misclassifications of class 0 (minority class)
class_weights = torch.tensor([1.0, len(y) / np.sum(y == 1)], dtype=torch.float32)  # Weight for class 0
criterion = nn.CrossEntropyLoss(weight=class_weights)  # Class weighting
optimizer = optim.Adam(model.parameters(), lr=0.001)

# --- 5. Training Loop ---
num_epochs = 30
for epoch in range(num_epochs):
    model.train()
    optimizer.zero_grad()

    # Forward pass
    out = model(train_data.x, train_data.edge_index)
    loss = criterion(out, train_data.y)

    # Backward pass
    loss.backward()
    optimizer.step()

    # Print loss every 10 epochs
    if (epoch + 1) % 10 == 0:
        print(f"Epoch {epoch+1}/{num_epochs}, Loss: {loss.item():.4f}")

# --- 6. Save the Model ---
torch.save(model.state_dict(), "backend/gnn_pipeline/data/model.pth")
print("[+] Model saved to backend/gnn_pipeline/data/model.pth")
