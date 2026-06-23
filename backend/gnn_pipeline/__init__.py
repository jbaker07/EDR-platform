import os

# torch / torch_geometric are heavy, optional deps. Guard them so the package
# imports without them (the torch-free utils, e.g. graph_archiver, still work).
# load_model() raises clearly if the GNN stack is genuinely needed but absent.
try:
    import torch
    from gnn_pipeline.model import GNNModel
    _GNN_STACK_AVAILABLE = True
except ImportError:  # pragma: no cover
    torch = None
    GNNModel = None
    _GNN_STACK_AVAILABLE = False

def load_model(path="backend/gnn_pipeline/model.pth", input_dim=128):
    if not _GNN_STACK_AVAILABLE:
        raise RuntimeError(
            "GNN model stack unavailable (install 'torch' and 'torch_geometric')."
        )
    model = GNNModel(input_dim=input_dim)
    if os.path.exists(path):
        model.load_state_dict(torch.load(path))
        print("[+] Loaded trained GNN model.")
    else:
        print("[!] model.pth not found — using untrained GNN model for development.")
    return model
