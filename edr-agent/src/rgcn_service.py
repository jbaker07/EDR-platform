# src/rgcn_service.py
# Lightweight R-GCN sidecar with two backends:
#  - NX (default): structural risk scoring via NetworkX (no heavy deps)
#  - PT/DGL (optional): hetero R-GCN skeleton you can enable later

from __future__ import annotations

import os
import time
import json
import hashlib
from typing import Dict, Any, List, Optional, Tuple

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

# ---------- Backend selection & optional deps ----------
BACKEND = os.getenv("RGCN_BACKEND", "nx").lower()

TORCH_AVAILABLE = False
DGL_AVAILABLE = False
NX_AVAILABLE = False

# Optional heavy deps (only if PT/DGL requested)
if BACKEND in ("pt", "torch", "pytorch", "dgl"):
    try:
        import torch  # type: ignore
        import torch.nn as nn  # type: ignore
        TORCH_AVAILABLE = True
        import dgl  # type: ignore
        import dgl.nn as dglnn  # type: ignore
        DGL_AVAILABLE = True
    except Exception as e:
        raise RuntimeError(
            f"PT/DGL backend requested (RGCN_BACKEND={BACKEND}) but import failed: {e}"
        )

# Always try to have numpy (used in both paths)
try:
    import numpy as np  # type: ignore
except Exception as e:
    raise RuntimeError(f"numpy is required: {e}")

# Optional NetworkX (for NX backend)
try:
    import networkx as nx  # type: ignore
    NX_AVAILABLE = True
except Exception:
    NX_AVAILABLE = False

# ---------- App ----------
app = FastAPI()

# ---------- Config ----------
DEVICE = os.getenv("RGCN_DEVICE", "cpu")
WEIGHTS = os.getenv("RGCN_WEIGHTS", "")
MODEL_NAME = os.path.basename(WEIGHTS) if WEIGHTS else "untrained"
APP_START = int(time.time())


# ---------- Helpers ----------
def ntype_from_id(nid: str) -> str:
    """Map node-id prefixes to canonical types."""
    if nid.startswith("proc:"):
        return "Process"
    if nid.startswith("file:"):
        return "File"
    if nid.startswith("ip:"):
        return "IP"
    if nid.startswith("user:"):
        return "User"
    if nid.startswith("domain:"):
        return "Domain"
    return "Other"


# ---------- Schemas ----------
class NodeIn(BaseModel):
    id: str
    kind: Optional[str] = None
    last_seen: Optional[int] = None
    trust_score: Optional[float] = None  # 0..1 where 1=trusted
    risk: Optional[float] = None         # 0..100 alternative field
    # other fields (exe, cmdline, etc.) may be present and are ignored by the service


class EdgeIn(BaseModel):
    source: str
    target: str
    rel: Optional[str] = None
    last_seen: Optional[int] = None
    weight: Optional[float | int] = 1.0


class InferIn(BaseModel):
    ts: Optional[int] = None
    nodes: List[NodeIn]
    edges: List[EdgeIn]


# =====================================================================
# NX BACKEND (default): structural scorer with no heavy dependencies
# =====================================================================

def _norm(d: Dict[str, float]) -> Dict[str, float]:
    if not d:
        return {}
    vals = list(d.values())
    mn, mx = min(vals), max(vals)
    if mx - mn < 1e-9:
        return {k: 0.0 for k in d}
    return {k: (v - mn) / (mx - mn) for k, v in d.items()}


def compute_scores_nx(payload: InferIn) -> Dict[str, float]:
    if not NX_AVAILABLE:
        raise HTTPException(status_code=500, detail="networkx not available")

    G = nx.DiGraph()

    # Nodes with a prior trust
    for n in payload.nodes:
        trust = (
            n.trust_score
            if n.trust_score is not None
            else 1.0 - (float(n.risk) / 100.0) if n.risk is not None
            else 1.0
        )
        G.add_node(n.id, kind=(n.kind or ntype_from_id(n.id)), trust=max(0.0, min(1.0, trust)))

    # Edges
    for e in payload.edges:
        G.add_edge(e.source, e.target, rel=e.rel or "related", w=(float(e.weight or 1.0)))

    if G.number_of_nodes() == 0:
        return {}

    # Structural signals
    indeg = {n: float(G.in_degree(n)) for n in G.nodes()}
    outdeg = {n: float(G.out_degree(n)) for n in G.nodes()}

    try:
        pr = nx.pagerank(G, alpha=0.85, max_iter=100)
    except Exception:
        pr = {n: 1.0 / G.number_of_nodes() for n in G.nodes()}

    # Neighbor risk: average (1 - trust) of neighbors
    neigh_risk: Dict[str, float] = {}
    for n in G.nodes():
        neigh = list(G.predecessors(n)) + list(G.successors(n))
        if not neigh:
            neigh_risk[n] = 0.0
        else:
            rr = [1.0 - float(G.nodes[m].get("trust", 1.0)) for m in neigh]
            neigh_risk[n] = sum(rr) / len(rr)

    # Normalize signals
    pr_n = _norm(pr)
    indegN = _norm(indeg)
    outdegN = _norm(outdeg)
    nriskN = _norm(neigh_risk)

    # Combine signals (0..1)
    scores: Dict[str, float] = {}
    for n in G.nodes():
        base_risk = 1.0 - float(G.nodes[n].get("trust", 1.0))
        s = (
            0.35 * pr_n.get(n, 0.0) +
            0.20 * indegN.get(n, 0.0) +
            0.20 * outdegN.get(n, 0.0) +
            0.15 * nriskN.get(n, 0.0) +
            0.10 * base_risk
        )
        scores[n] = float(max(0.0, min(1.0, s)))
    return scores


# =====================================================================
# PT/DGL BACKEND (optional): hetero R-GCN skeleton (enable later)
# =====================================================================

MODEL = None
MODEL_SHA = ""


def _ensure_pt_dgl():
    if not (TORCH_AVAILABLE and DGL_AVAILABLE):
        raise HTTPException(status_code=500, detail="PT/DGL not available in this build")


def build_hetero_graph(payload: InferIn):
    """Build a DGL heterograph; only called when PT/DGL is enabled."""
    _ensure_pt_dgl()
    import torch  # local import to keep outer module import-safe
    import dgl

    nodes = payload.nodes
    edges = payload.edges

    # Per-type node lists
    per_type: Dict[str, List[str]] = {}
    for n in nodes:
        t = (n.kind or ntype_from_id(n.id))
        per_type.setdefault(t, []).append(n.id)

    # Index maps
    nidx: Dict[str, Dict[str, int]] = {t: {nid: i for i, nid in enumerate(ids)} for t, ids in per_type.items()}

    # Edge tuples per (src_t, rel, dst_t)
    rel_edges: Dict[Tuple[str, str, str], List[Tuple[int, int]]] = {}
    for e in edges:
        s_t = ntype_from_id(e.source)
        d_t = ntype_from_id(e.target)
        key = (s_t, e.rel or "related", d_t)
        s_i = nidx.get(s_t, {}).get(e.source, None)
        d_i = nidx.get(d_t, {}).get(e.target, None)
        if s_i is None or d_i is None:
            continue
        rel_edges.setdefault(key, []).append((s_i, d_i))

    # Build heterograph
    graph_data = {}
    for (s_t, rel, d_t), pairs in rel_edges.items():
        if not pairs:
            continue
        src_idx = [p[0] for p in pairs]
        dst_idx = [p[1] for p in pairs]
        graph_data[(s_t, rel, d_t)] = (torch.tensor(src_idx), torch.tensor(dst_idx))

    g = dgl.heterograph(graph_data)
    # Return also the per-type id list for output re-mapping
    return g, per_type


def compute_degrees(g, per_type_ids: Dict[str, List[str]]) -> Dict[str, np.ndarray]:
    """Compute [in_deg, out_deg] per node-type; DGL aggregates across all rels."""
    _ensure_pt_dgl()
    import dgl

    feats: Dict[str, np.ndarray] = {}
    for ntype, ids in per_type_ids.items():
        num = len(ids)
        if ntype not in g.ntypes:
            # Ensure node type exists (0 edges case)
            g.add_nodes(num, ntype=ntype)
        if num == 0:
            feats[ntype] = np.zeros((0, 2), dtype=np.float32)
            continue
        in_deg = g.in_degrees(ntype=ntype).numpy().astype(np.float32)
        out_deg = g.out_degrees(ntype=ntype).numpy().astype(np.float32)
        f = np.stack([np.log1p(in_deg), np.log1p(out_deg)], axis=1)
        feats[ntype] = f
    return feats


def recency_and_risk_vec(payload: InferIn, per_type_ids: Dict[str, List[str]]) -> Dict[str, np.ndarray]:
    """[recency, risk_prior] per node-type: recency in [0..1], risk_prior in [0..1]."""
    now = payload.ts or int(time.time())
    meta = {n.id: n for n in payload.nodes}
    out: Dict[str, np.ndarray] = {}
    for ntype, ids in per_type_ids.items():
        rec = np.zeros((len(ids), 2), dtype=np.float32)
        for i, nid in enumerate(ids):
            n = meta.get(nid)
            # recency: minutes since last_seen capped at 60, normalized 0..1 (0=very recent)
            if n and n.last_seen:
                delta = max(0, now - int(n.last_seen))
            else:
                delta = 3600
            recency = min(60.0, delta / 60.0) / 60.0
            # risk prior: 1 - trust_score (or from risk field)
            if n and n.trust_score is not None:
                risk_prior = 1.0 - float(n.trust_score)
            elif n and n.risk is not None:
                risk_prior = float(n.risk) / 100.0
            else:
                risk_prior = 0.0
            rec[i, 0] = recency
            rec[i, 1] = max(0.0, min(1.0, risk_prior))
        out[ntype] = rec
    return out


# ---- Simple hetero R-GCN scaffold ----
if TORCH_AVAILABLE and DGL_AVAILABLE:
    import torch
    import torch.nn as nn
    import dgl
    import dgl.nn as dglnn

    class TypeLinear(nn.Module):
        def __init__(self, in_dim: int, out_dim: int, ntypes: List[str]):
            super().__init__()
            self.lins = nn.ModuleDict({t: nn.Linear(in_dim, out_dim) for t in ntypes})

        def forward(self, h: Dict[str, torch.Tensor]) -> Dict[str, torch.Tensor]:
            return {t: torch.relu(self.lins[t](x)) for t, x in h.items()}

    class RGCNModel(nn.Module):
        def __init__(self, ntypes: List[str], rel_names: List[Tuple[str, str, str]], in_dim=4, hid=32):
            super().__init__()
            self.ntypes = ntypes
            self.rel_names = rel_names
            self.in_proj = TypeLinear(in_dim, hid, ntypes)
            conv1 = {r: dglnn.GraphConv(hid, hid, norm="both") for r in rel_names}
            conv2 = {r: dglnn.GraphConv(hid, hid, norm="both") for r in rel_names}
            self.hconv1 = dglnn.HeteroGraphConv(conv1, aggregate="sum")
            self.hconv2 = dglnn.HeteroGraphConv(conv2, aggregate="sum")
            self.out_proj = nn.ModuleDict({t: nn.Linear(hid, 1) for t in ntypes})

        def forward(self, g: dgl.DGLHeteroGraph, feats: Dict[str, torch.Tensor]) -> Dict[str, torch.Tensor]:
            h = self.in_proj(feats)
            h = self.hconv1(g, h)
            h = {t: torch.relu(x) for t, x in h.items()}
            h = self.hconv2(g, h)
            h = {t: torch.relu(x) for t, x in h.items()}
            out = {t: torch.sigmoid(self.out_proj[t](x)).squeeze(-1) for t, x in h.items()}
            return out

    MODEL: Optional[RGCNModel] = None
else:
    # Keep names defined so references exist; they won't be used on NX backend.
    RGCNModel = None  # type: ignore
    MODEL = None


def load_model(ntypes: List[str], rels: List[Tuple[str, str, str]], weights_path: Optional[str]):
    global MODEL, MODEL_SHA, MODEL_NAME
    _ensure_pt_dgl()
    import torch

    MODEL = RGCNModel(ntypes, rels).to(DEVICE)  # type: ignore
    if weights_path and os.path.exists(weights_path):
        sd = torch.load(weights_path, map_location=DEVICE)
        MODEL.load_state_dict(sd, strict=False)  # type: ignore
        with open(weights_path, "rb") as f:
            MODEL_SHA = hashlib.sha256(f.read()).hexdigest()[:12]
        MODEL_NAME = os.path.basename(weights_path)
    else:
        MODEL_SHA = "dev-random"


def ensure_model(g):
    """Lazy-init the PT/DGL model from the seen graph schema."""
    _ensure_pt_dgl()
    if MODEL is not None:
        return
    ntypes = list(g.ntypes)
    rels = list(g.canonical_etypes)
    load_model(ntypes, rels, WEIGHTS)


# =====================================================================
# Endpoints
# =====================================================================

@app.get("/healthz")
def healthz():
    return {
        "ok": True if (BACKEND == "nx") else (MODEL is not None),
        "backend": BACKEND,
        "torch": TORCH_AVAILABLE,
        "dgl": DGL_AVAILABLE,
        "device": str(DEVICE),
        "model": MODEL_NAME,
        "sha": MODEL_SHA,
        "since": APP_START,
    }


@app.post("/hotload")
def hotload(body: dict):
    p = body.get("path")
    if not p or not os.path.exists(p):
        raise HTTPException(400, "weights path missing or not found")

    if BACKEND == "nx":
        # Nothing to hotload for NX; accept for API consistency.
        return {"ok": True, "note": "NX backend does not use weights"}

    _ensure_pt_dgl()
    # Reinitialize from last seen schema if available
    if MODEL is None:
        global WEIGHTS, MODEL_NAME
        WEIGHTS = p
        MODEL_NAME = os.path.basename(p)
        return {"ok": True, "note": "will load on next /infer"}
    # Warm reload
    load_model(MODEL.ntypes, MODEL.rel_names, p)  # type: ignore
    return {"ok": True, "model": MODEL_NAME, "sha": MODEL_SHA}


@app.post("/infer")
def infer(payload: InferIn):
    t0 = time.time()
    if not payload.nodes:
        return {"scores": {}, "backend": BACKEND, "elapsed_ms": 0}

    # NX path (default)
    if BACKEND == "nx":
        scores = compute_scores_nx(payload)
        return {
            "scores": scores,
            "backend": "nx",
            "elapsed_ms": int((time.time() - t0) * 1000),
        }

    # PT/DGL path (optional)
    if BACKEND in ("pt", "torch", "pytorch", "dgl"):
        _ensure_pt_dgl()
        import torch  # local import for safety

        g, per_type_ids = build_hetero_graph(payload)
        ensure_model(g)

        # Build 4-dim features per type: [in_deg, out_deg] + [recency, risk_prior]
        deg = compute_degrees(g, per_type_ids)
        rr = recency_and_risk_vec(payload, per_type_ids)

        feats: Dict[str, torch.Tensor] = {}
        for t in g.ntypes:
            a = deg.get(t, np.zeros((g.num_nodes(t), 2), dtype=np.float32))
            b = rr.get(t, np.zeros((g.num_nodes(t), 2), dtype=np.float32))
            x = np.concatenate([a, b], axis=1)
            feats[t] = torch.from_numpy(x).to(DEVICE)

        # Forward
        MODEL.eval()  # type: ignore
        outs = MODEL(g.to(DEVICE), feats)  # type: ignore

        # Flatten to {node_id: score}
        result: Dict[str, float] = {}
        for t, vec in outs.items():
            ids = per_type_ids.get(t, [])
            scores = vec.detach().cpu().numpy().tolist()
            for nid, sc in zip(ids, scores):
                result[nid] = float(max(0.0, min(1.0, sc)))

        return {
            "scores": result,
            "backend": "pt",
            "elapsed_ms": int((time.time() - t0) * 1000),
        }

    # Unknown backend
    raise HTTPException(status_code=400, detail=f"Unknown backend: {BACKEND}")
