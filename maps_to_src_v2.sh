#!/usr/bin/env bash
set -u
set -o pipefail
trap '' PIPE   # avoid SIGPIPE 141 on broken pipes

maps=(file_events net_events events edr_events_rb wx_events)

printf "map_name,map_id,prog_id,prog_name,btf_obj,src_path,pinned\n"

for NAME in "${maps[@]}"; do
  # all map IDs that match this name
  for MID in $(sudo bpftool -jp map show | jq -r --arg n "$NAME" '.[] | select(.name==$n) | .id'); do
    PIN=$(sudo bpftool -jp map show id "$MID" \
          | jq -r 'if type=="array" then (.[0].pinned // "") else (.pinned // "") end')

    # programs that reference this map
    sudo bpftool -jp prog show \
    | jq -r --argjson m "$MID" '.[] | select(.map_ids and (.map_ids | index($m))) | [.id, .name, (.btf_id // 0)] | @tsv' \
    | while IFS=$'\t' read -r PID PNAME BTFID; do
        # BTF object (often empty/vmlinux for CO-RE)
        OBJ=$(sudo bpftool -jp btf show id "$BTFID" 2>/dev/null \
              | jq -r 'if type=="array" then (.[0].name // "") else (.name // "") end')
        [[ -z "$OBJ" || "$OBJ" == "null" ]] && OBJ="(no BTF obj name)"

        # extract only the file path from linum (handle multiple formats)
        SRC=$(
          sudo bpftool prog dump xlated id "$PID" linum 2>/dev/null | \
          awk '
            # Format A: "[... file:/path, line_num:... ]"
            match($0, /file:[[:space:]]*([^,]+),/, m)                     { print m[1]; exit }
            # Format B: "[/path:123]"
            match($0, /\[([^][]+):[0-9]+\]/, m)                           { print m[1]; exit }
            # Format C: "; file: /path:123"
            match($0, /;[[:space:]]*file:[[:space:]]*([^:]+):[0-9]+/, m)  { print m[1]; exit }
          '
        )
        [[ -z "$SRC" ]] && SRC="(no source path in linum)"

        printf "%s,%s,%s,%s,%s,%s,%s\n" \
               "$NAME" "$MID" "$PID" "$PNAME" "$OBJ" "$SRC" "${PIN:-}"
      done
  done
done
