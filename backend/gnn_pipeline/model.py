import torch
from torch.nn import Linear, ReLU
from torch_geometric.nn import GCNConv

class GNNModel(torch.nn.Module):
    def __init__(self, in_channels: int, hidden_channels: int, out_channels: int = 2):
        super().__init__()
        self.conv1 = GCNConv(in_channels, hidden_channels)
        self.conv2 = GCNConv(hidden_channels, hidden_channels)
        self.linear = Linear(hidden_channels, out_channels)
        self.relu = ReLU()

    def forward(self, x, edge_index):
        x = self.relu(self.conv1(x, edge_index))
        x = self.relu(self.conv2(x, edge_index))
        return self.linear(x)