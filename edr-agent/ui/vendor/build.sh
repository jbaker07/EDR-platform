#!/bin/bash
# Rebuilds the agent UI's vendored files from the dashboard's installed packages, so the
# UI loads nothing from a public CDN. Run from the repo root after `npm ci` in dashboard/.
#   tailwind.css    Tailwind 4 run over index.html, app.js and auth.js
#   d3.min.js       the D3 parts graph.html uses (d3_entry.js), as the global d3
#   chart.umd.js, chartjs-adapter-date-fns.bundle.min.js   copied from node_modules
set -eu
cd "$(dirname "$0")"
NM=../../../dashboard/node_modules
NODE_PATH=$NM node -e '
const fs = require("fs"), postcss = require("postcss"), tw = require("@tailwindcss/postcss");
postcss([tw({ optimize: { minify: true } })])
  .process(fs.readFileSync("tailwind.input.css", "utf8"), { from: "tailwind.input.css", to: "tailwind.css" })
  .then((r) => fs.writeFileSync("tailwind.css", r.css));'
NODE_PATH=$NM "$NM/.bin/esbuild" d3_entry.js --bundle --format=iife --global-name=d3 --minify \
  --legal-comments=inline --outfile=d3.min.js --log-level=warning
cp "$NM/chart.js/dist/chart.umd.js" chart.umd.js
cp "$NM/chartjs-adapter-date-fns/dist/chartjs-adapter-date-fns.bundle.min.js" .
ls -l tailwind.css d3.min.js chart.umd.js chartjs-adapter-date-fns.bundle.min.js
