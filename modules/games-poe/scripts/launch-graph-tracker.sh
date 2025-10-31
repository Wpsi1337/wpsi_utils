#!/usr/bin/env bash
set -euo pipefail

GRAPH_TRACKER_DIR=${GRAPH_TRACKER_DIR:-/home/wps/graph_tracker}
LAUNCHER=${GRAPH_TRACKER_LAUNCHER:-run_tracker.sh}

if [ ! -d "$GRAPH_TRACKER_DIR" ]; then
    printf 'Graph tracker directory not found: %s\n' "$GRAPH_TRACKER_DIR" >&2
    exit 1
fi

if [ ! -x "$GRAPH_TRACKER_DIR/$LAUNCHER" ]; then
    printf 'Graph tracker launcher missing or not executable: %s/%s\n' "$GRAPH_TRACKER_DIR" "$LAUNCHER" >&2
    exit 1
fi

printf 'Launching PoE graph tracker...\n'
cd "$GRAPH_TRACKER_DIR"
exec "$GRAPH_TRACKER_DIR/$LAUNCHER"
