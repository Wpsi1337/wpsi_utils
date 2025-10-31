#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)

if ! command -v cargo >/dev/null 2>&1; then
    printf 'cargo is required to run wpsi_utils. Install Rust from https://rustup.rs/ first.\n' >&2
    exit 1
fi

cd "$REPO_ROOT"
exec cargo run -p toolbox-tui -- "$@"
