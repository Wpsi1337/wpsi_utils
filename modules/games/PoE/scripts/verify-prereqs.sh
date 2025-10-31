#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
MODULE_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
TOOLS_DIR="$MODULE_ROOT/tools"

SOURCE_ZONE="$TOOLS_DIR/zone_changer"
SOURCE_GRAPH="$TOOLS_DIR/graph_tracker"

if [ ! -d "$SOURCE_ZONE" ]; then
    printf 'Repo copy of zone_changer missing at %s\n' "$SOURCE_ZONE" >&2
    exit 1
fi

if [ ! -d "$SOURCE_GRAPH" ]; then
    printf 'Repo copy of graph_tracker missing at %s\n' "$SOURCE_GRAPH" >&2
    exit 1
fi

TARGET_ZONE=${ZONE_CHANGER_DIR:-"$HOME/zone_changer"}
TARGET_GRAPH=${GRAPH_TRACKER_DIR:-"$HOME/graph_tracker"}
FORCE=${POE_SYNC_FORCE:-0}

sync_dir() {
    local source=$1
    local target=$2
    local label=$3

    if [ -d "$target" ] && [ "$FORCE" != "1" ]; then
        printf '%s already exists at %s (set POE_SYNC_FORCE=1 to overwrite).\n' "$label" "$target"
        return
    fi

    rm -rf "$target"
    mkdir -p "$target"
    cp -a "$source/." "$target/"
    printf 'Restored %s to %s\n' "$label" "$target"
}

sync_dir "$SOURCE_ZONE" "$TARGET_ZONE" "zone_changer"
sync_dir "$SOURCE_GRAPH" "$TARGET_GRAPH" "graph_tracker"

printf 'PoE helper assets are ready.\n'
