#!/usr/bin/env bash
set -euo pipefail

ZONE_CHANGER_DIR=${ZONE_CHANGER_DIR:-/home/wps/zone_changer}
SCRIPT=${ZONE_CHANGER_SCRIPT:-poe2_zone_watcher.py}

if [ ! -d "$ZONE_CHANGER_DIR" ]; then
    printf 'Zone changer directory not found: %s\n' "$ZONE_CHANGER_DIR" >&2
    exit 1
fi

if [ ! -f "$ZONE_CHANGER_DIR/$SCRIPT" ]; then
    printf 'Zone changer script missing: %s/%s\n' "$ZONE_CHANGER_DIR" "$SCRIPT" >&2
    exit 1
fi

printf 'Launching PoE zone changer...\n'
exec python3 "$ZONE_CHANGER_DIR/$SCRIPT"
