# Data path
kernel (eBPF) -> maps (ringbuf/perf) -> tail (normalize to CommonEvent) ->
[ rules | baselines | R-GCN ] -> alerts + incidents -> outputs (stdout, sqlite, files)
