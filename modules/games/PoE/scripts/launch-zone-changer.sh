#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
MODULE_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
TOOLS_DIR="$MODULE_ROOT/tools"

FALLBACK_DIR="$TOOLS_DIR/zone_changer"
ZONE_CHANGER_DIR=${ZONE_CHANGER_DIR:-"$HOME/zone_changer"}
ZONE_SCRIPT=${ZONE_CHANGER_SCRIPT:-poe2_zone_watcher.py}

if [ ! -d "$ZONE_CHANGER_DIR" ]; then
    printf 'Zone changer directory not found at %s, falling back to repo copy.\n' "$ZONE_CHANGER_DIR"
    ZONE_CHANGER_DIR="$FALLBACK_DIR"
fi

if [ ! -f "$ZONE_CHANGER_DIR/$ZONE_SCRIPT" ]; then
    printf 'Zone changer script missing: %s/%s\n' "$ZONE_CHANGER_DIR" "$ZONE_SCRIPT" >&2
    printf 'Run scripts/verify-prereqs.sh to install the helper locally.\n' >&2
    exit 1
fi

printf 'Launching PoE zone changer from %s ...\n' "$ZONE_CHANGER_DIR"
exec python3 "$ZONE_CHANGER_DIR/$ZONE_SCRIPT"
