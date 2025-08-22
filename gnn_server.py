from flask import Flask, request, jsonify
import torch
import torch.nn.functional as F
import os

AUTH_TOKEN = os.environ.get("GNN_AUTH_TOKEN", "super-secret-token")
MODEL_PATH = "models/gnn_model.pt"

class SimpleGNN(torch.nn.Module):
    def __init__(self, input_dim):
        super().__init__()
        self.fc1 = torch.nn.Linear(input_dim, 16)
        self.fc2 = torch.nn.Linear(16, 1)

    def forward(self, x):
        x = F.relu(self.fc1(x))
        return torch.sigmoid(self.fc2(x))

model = SimpleGNN(input_dim=10)  # Adjust as needed
model.load_state_dict(torch.load(MODEL_PATH))
model.eval()

app = Flask(__name__)

@app.route("/gnn", methods=["POST"])
def gnn_score():
    data = request.json
    if not data or data.get("auth") != AUTH_TOKEN:
        return jsonify({"error": "unauthorized"}), 403

    features = data.get("telemetry", {})
    input_tensor = torch.tensor([[float(v) for v in features.values()]], dtype=torch.float32)

    with torch.no_grad():
        score = model(input_tensor).item()

    return jsonify({"score": round(min(score, 1.0), 4)})

if __name__ == "__main__":
    app.run(host="127.0.0.1", port=5051)
