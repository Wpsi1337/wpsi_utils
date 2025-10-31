#!/usr/bin/env bash
set -euo pipefail

cat <<'USAGE'
This is the developer entrypoint for toolbox.
Recommended workflow:
  cargo run -p tui -- --config ./config/toolbox.toml
Or run the CLI with extra flags:
  cargo run -p cli -- --list-modules
Extend this script with your favorite dev shortcuts when features exist.
USAGE
