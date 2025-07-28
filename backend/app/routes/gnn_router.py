from fastapi import APIRouter, Query
from pydantic import BaseModel
import torch
import pickle
import numpy as np
from torch_geometric.data import Data
from app.memory.memory_manager import memory_manager
from app.services.trust_engine_final import adjust_trust_from_gnn
from app.models.gnn_model import GCN
from app.services.trust_engine_final import adjust_trust_from_gnn

router = APIRouter()

class TelemetryInput(BaseModel):
    hostname: str
    pid: int
    cpu_percent: float
    memory: float
    risk_score: float
    risk_level: str
    source: str

MODEL_PATHS = {
    "wave1": "backend/gnn_pipeline/models/gnn_model.pt",
    "wave2": "backend/gnn_pipeline/models/gnn_model_wave2.pt",
    "wave3": "backend/gnn_pipeline/models/gnn_model_wave3.pt",
    "wave4": "backend/gnn_pipeline/models/gnn_model_wave4.pt",
    "wave5": "backend/gnn_pipeline/models/gnn_model_wave5.pt",
    "wave6": "backend/gnn_pipeline/models/gnn_model_wave6.pt",
    "wave7": "backend/gnn_pipeline/models/gnn_model_wave7.pt",
    "wave8": "backend/gnn_pipeline/models/gnn_model_wave8.pt",
    "wave9": "backend/gnn_pipeline/models/gnn_model_wave9.pt",
    "wave10": "backend/gnn_pipeline/models/gnn_model_wave10.pt" #✅ Add this
}


@router.post("/api/gnn/predict")
def predict_telemetry(data: TelemetryInput, version: str = Query("wave2")):
    model_path = MODEL_PATHS.get(version)
    if not model_path:
        return {"error": f"Invalid model version: {version}"}

    try:
        model = GCN(in_channels=6, hidden_channels=16, out_channels=2)
        model.load_state_dict(torch.load(model_path))
        model.eval()
    except Exception as e:
        return {"error": f"Failed to load model: {str(e)}"}

    try:
        # === Step 1: Try to get real parent node from memory ===
        parent_embedding = memory_manager.get_last_embedding(data.pid)  # You store this per `pid`

        if parent_embedding:
            parent_node = parent_embedding
        else:
            parent_node = [0.0, 5.0, 1_000_000.0, 5.0, 0, 0]  # fallback synthetic

        # === Step 2: Build target process node ===
        current_node = [
            1.0,  # degree
            data.cpu_percent,
            data.memory,
            data.risk_score,
            1 if data.risk_level.lower() in ["high", "critical"] else 0,
            1 if "malicious" in data.source else 0
        ]

        x = torch.tensor([current_node, parent_node], dtype=torch.float32)
        edge_index = torch.tensor([[1, 0], [0, 1]], dtype=torch.long)  # 2-node bidirectional edge

        # === Step 3: Inference ===
        graph_data = Data(x=x, edge_index=edge_index)

        with torch.no_grad():
            out = model(graph_data.x, graph_data.edge_index)
            pred = torch.argmax(out[0]).item()
            confidence = torch.softmax(out[0], dim=0)[pred].item()

        # Cache this node’s embedding
        memory_manager.store_embedding(data.hostname, current_node)

        # Adjust trust
        trust_score = adjust_trust_from_gnn(data.hostname, pred, confidence)

        return {
            "hostname": data.hostname,
            "model_version": version,
            "prediction": "malicious" if pred == 1 else "benign",
            "confidence": round(confidence, 4),
            "trust_score": trust_score
        }

    except Exception as e:
        return {"error": f"Failed GNN inference: {str(e)}"}
