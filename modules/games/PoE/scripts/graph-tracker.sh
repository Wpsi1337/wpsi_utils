#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
VERIFY_SCRIPT="$SCRIPT_DIR/verify-prereqs.sh"

"$VERIFY_SCRIPT"

GRAPH_TRACKER_DIR=${GRAPH_TRACKER_DIR:-"$HOME/graph_tracker"}
TRACKER_LAUNCHER=${GRAPH_TRACKER_LAUNCHER:-run_tracker.sh}
FALLBACK_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../tools/graph_tracker" && pwd)

if [ ! -d "$GRAPH_TRACKER_DIR" ]; then
    printf 'Graph tracker directory missing at %s, using bundled copy.\n' "$GRAPH_TRACKER_DIR"
    GRAPH_TRACKER_DIR="$FALLBACK_DIR"
fi

if [ ! -x "$GRAPH_TRACKER_DIR/$TRACKER_LAUNCHER" ]; then
    printf 'Launcher %s not executable in %s.\n' "$TRACKER_LAUNCHER" "$GRAPH_TRACKER_DIR" >&2
    printf 'Re-run with POE_SYNC_FORCE=1 to refresh the helper.\n' >&2
    exit 1
fi

printf 'Launching PoE graph tracker from %s...\n' "$GRAPH_TRACKER_DIR"
cd "$GRAPH_TRACKER_DIR"
exec "$GRAPH_TRACKER_DIR/$TRACKER_LAUNCHER"
