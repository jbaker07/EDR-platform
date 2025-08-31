`tail` opens both ringbufs (/sys/fs/bpf/edr/events, /sys/fs/bpf/edr/edr_events_rb)
and perf event arrays (file_events, net_events, wx_events), converts each record
to CommonEvent JSON, prints to stdout, and (optionally) writes to SQLite.
Flags:
  --rb-only / --perf-only
  --sqlite data/sqlite/edr.db
  --pretty
  --rules edr-agent/etc/rules/*.yaml
