import torch
import torch.optim as optim
import torch.nn as nn
from sklearn.preprocessing import StandardScaler, OneHotEncoder
from sklearn.model_selection import train_test_split
import pandas as pd
import numpy as np

# --- 1. Load and preprocess the dataset ---
# Load the dataset
df = pd.read_csv('data/merged_scored_labeled.csv')

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

# --- 3. Define a Simple Neural Network Model ---
class SimpleNN(nn.Module):
    def __init__(self, input_dim, hidden_dim, output_dim):
        super(SimpleNN, self).__init__()
        self.fc1 = nn.Linear(input_dim, hidden_dim)
        self.fc2 = nn.Linear(hidden_dim, output_dim)
        self.relu = nn.ReLU()

    def forward(self, x):
        x = self.fc1(x)
        x = self.relu(x)
        x = self.fc2(x)
        return x

# --- 4. Initialize Model, Loss, and Optimizer ---
input_dim = X_train.shape[1]  # Number of features
hidden_dim = 64
output_dim = len(np.unique(y))  # Number of classes (malicious/benign)

model = SimpleNN(input_dim, hidden_dim, output_dim)

# Loss and Optimizer
criterion = nn.CrossEntropyLoss()
optimizer = optim.Adam(model.parameters(), lr=0.001)

# --- 5. Training Loop ---
num_epochs = 30
for epoch in range(num_epochs):
    model.train()
    optimizer.zero_grad()

    # Forward pass
    out = model(X_train)
    loss = criterion(out, y_train)

    # Backward pass
    loss.backward()
    optimizer.step()

    if (epoch + 1) % 10 == 0:
        print(f"Epoch {epoch+1}/{num_epochs}, Loss: {loss.item():.4f}")

# --- 6. Evaluate Model ---
model.eval()
with torch.no_grad():
    y_pred = model(X_test)
    _, predicted_classes = torch.max(y_pred, 1)

# --- 7. Output Predictions ---
print(f"Predicted Classes: {predicted_classes.numpy()}")


from sklearn.metrics import accuracy_score, precision_score, recall_score, f1_score, confusion_matrix

# Convert the torch tensors to numpy arrays for scikit-learn compatibility
y_test_numpy = y_test.numpy()
predictions_numpy = predicted_classes.numpy()

# Calculate the metrics
accuracy = accuracy_score(y_test_numpy, predictions_numpy)
precision = precision_score(y_test_numpy, predictions_numpy)
recall = recall_score(y_test_numpy, predictions_numpy)
f1 = f1_score(y_test_numpy, predictions_numpy)
conf_matrix = confusion_matrix(y_test_numpy, predictions_numpy)

# Output the results
print(f"Accuracy: {accuracy:.4f}")
print(f"Precision: {precision:.4f}")
print(f"Recall: {recall:.4f}")
print(f"F1-Score: {f1:.4f}")
print(f"Confusion Matrix:\n{conf_matrix}")
