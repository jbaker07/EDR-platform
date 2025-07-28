
import torch
import torch.nn as nn
import torch.nn.functional as F
import pickle


class GNNTrustModel(nn.Module):
    def __init__(self, input_dim=128, hidden_dim=64, output_dim=1):
        super(GNNTrustModel, self).__init__()
        self.fc1 = nn.Linear(input_dim, hidden_dim)
        self.fc2 = nn.Linear(hidden_dim, output_dim)

    def forward(self, x):
        x = F.relu(self.fc1(x))
        x = torch.sigmoid(self.fc2(x))  # Confidence score between 0 and 1
        return x


def load_model(model_path="gnn_model.pt", device="cpu"):
    model = GNNTrustModel()
    model.load_state_dict(torch.load(model_path, map_location=device))
    model.eval()
    return model


def score_node(graph_path, node_id, model, device="cpu"):
    with open(graph_path, "rb") as f:
        graph = pickle.load(f)

    if node_id not in graph.nodes:
        raise ValueError(f"Node ID {node_id} not found in the graph.")

    node_data = graph.nodes[node_id].get("features")
    if node_data is None:
        raise ValueError(f"No 'features' attribute found for node {node_id}.")

    x = torch.tensor(node_data, dtype=torch.float32).unsqueeze(0).to(device)
    model.to(device)
    with torch.no_grad():
        score = model(x)
    return score.item()
