import torch
import os
from model import GNNModel

def load_model(path="backend/gnn_pipeline/model.pth", input_dim=128):
    model = GNNModel(input_dim=input_dim)
    if os.path.exists(path):
        model.load_state_dict(torch.load(path))
        model.eval()
        print("[+] GNN model loaded from disk.")
    else:
        print("[!] model.pth not found — using untrained GNN model.")
    return model
