Nodes: process(pid@boot_id), file(inode@fsid), socket(fd@pid), ipv4, ipv6, host
Edges: opened, executed, forked, connected, dst
Temporal: keep timestamps; apply EWMA for feature decay
Features: see analytics/config/gnn.yaml
