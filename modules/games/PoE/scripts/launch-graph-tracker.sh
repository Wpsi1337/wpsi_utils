#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
MODULE_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
TOOLS_DIR="$MODULE_ROOT/tools"

FALLBACK_DIR="$TOOLS_DIR/graph_tracker"
GRAPH_TRACKER_DIR=${GRAPH_TRACKER_DIR:-"$HOME/graph_tracker"}
LAUNCHER=${GRAPH_TRACKER_LAUNCHER:-run_tracker.sh}

if [ ! -d "$GRAPH_TRACKER_DIR" ]; then
    printf 'Graph tracker directory not found at %s, falling back to repo copy.\n' "$GRAPH_TRACKER_DIR"
    GRAPH_TRACKER_DIR="$FALLBACK_DIR"
fi

if [ ! -x "$GRAPH_TRACKER_DIR/$LAUNCHER" ]; then
    printf 'Graph tracker launcher missing or not executable: %s/%s\n' "$GRAPH_TRACKER_DIR" "$LAUNCHER" >&2
    printf 'Run scripts/verify-prereqs.sh to install the helper locally.\n' >&2
    exit 1
fi

printf 'Launching PoE graph tracker from %s ...\n' "$GRAPH_TRACKER_DIR"
cd "$GRAPH_TRACKER_DIR"
exec "$GRAPH_TRACKER_DIR/$LAUNCHER"
