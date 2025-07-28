import networkx as nx
import pandas as pd
import torch
from torch_geometric.utils import from_networkx
from backend.telem.telemetry_enricher import enrich_telemetry_with_tags

def build_graph_from_csv(csv_path: str, output_path: str):
    df = pd.read_csv(csv_path)
    G = nx.DiGraph()

    for idx, row in df.iterrows():
        telemetry = row.to_dict()
        enriched = enrich_telemetry_with_tags(telemetry)
        G.add_node(idx, features=torch.tensor(list(enriched.values()), dtype=torch.float))

    for i in range(len(df) - 1):
        G.add_edge(i, i + 1)

    nx.write_gpickle(G, output_path)