#!/usr/bin/env sh

hyperfine \
  "meowj earth_meteorite_landings.json" \
  "gron earth_meteorite_landings.json" \
  "tabbyj --file earth_meteorite_landings.json" \
  "catj earth_meteorite_landings.json" \
  --warmup 10 \
  --export-markdown BENCH.md
