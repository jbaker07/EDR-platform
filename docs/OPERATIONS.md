## Verify kernel wiring (you already do this)
find /sys/fs/bpf/edr -maxdepth 2 -type f -printf "[pin] %p\n" | sort

## Start tail (when implemented)
edr-agent/cmd/tail/tail --pretty

## Start GNN service (when implemented)
python3 analytics/gnn/service.py --config analytics/config/gnn.yaml
