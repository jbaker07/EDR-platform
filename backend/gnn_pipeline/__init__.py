import os
import torch
from gnn_pipeline.model import GNNModel

def load_model(path="backend/gnn_pipeline/model.pth", input_dim=128):
    model = GNNModel(input_dim=input_dim)
    if os.path.exists(path):
        model.load_state_dict(torch.load(path))
        print("[+] Loaded trained GNN model.")
    else:
        print("[!] model.pth not found — using untrained GNN model for development.")
    return model
