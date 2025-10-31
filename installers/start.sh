#!/usr/bin/env bash
set -euo pipefail

cat <<'USAGE'
This is the placeholder installer for toolbox (stable channel).
Build locally with:
  cargo build --workspace
Run the TUI after building:
  cargo run -p tui
Add your distribution-specific setup steps here once the project has real binaries.
USAGE
