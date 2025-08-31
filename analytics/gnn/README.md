R-GCN microservice:
- consumes normalized CommonEvent stream (stdin, file, or UNIX socket)
- maintains a dynamic heterogeneous graph
- runs R-GCN inference to score nodes/edges (risk in [0,1])
- emits risk annotations back to the pipeline with explanations
Artifacts:
- `r_gcn.py` (model definition)
- `service.py` (inference server; incremental updates)
- `graph_schema.md` (node/edge/feature definitions)
Config at analytics/config/gnn.yaml
