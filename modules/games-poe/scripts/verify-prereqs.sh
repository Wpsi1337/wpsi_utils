#!/usr/bin/env bash
set -euo pipefail

ZONE_CHANGER_DIR=${ZONE_CHANGER_DIR:-/home/wps/zone_changer}
GRAPH_TRACKER_DIR=${GRAPH_TRACKER_DIR:-/home/wps/graph_tracker}

missing=false

if [ ! -d "$ZONE_CHANGER_DIR" ]; then
    printf 'Missing zone changer directory: %s\n' "$ZONE_CHANGER_DIR" >&2
    missing=true
fi

if [ ! -d "$GRAPH_TRACKER_DIR" ]; then
    printf 'Missing graph tracker directory: %s\n' "$GRAPH_TRACKER_DIR" >&2
    missing=true
fi

if [ "$missing" = true ]; then
    printf 'Please adjust ZONE_CHANGER_DIR/GRAPH_TRACKER_DIR or clone the projects before running.\n' >&2
    exit 1
fi

printf 'PoE helper directories detected.\n'
