#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
VERIFY_SCRIPT="$SCRIPT_DIR/verify-prereqs.sh"

# Ensure the helper projects are available locally (no overwrite by default).
"$VERIFY_SCRIPT"

ZONE_CHANGER_DIR=${ZONE_CHANGER_DIR:-"$HOME/zone_changer"}
ZONE_SCRIPT=${ZONE_CHANGER_SCRIPT:-poe2_zone_watcher.py}
FALLBACK_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../tools/zone_changer" && pwd)

if [ ! -d "$ZONE_CHANGER_DIR" ]; then
    printf 'Zone changer directory missing at %s, using bundled copy.\n' "$ZONE_CHANGER_DIR"
    ZONE_CHANGER_DIR="$FALLBACK_DIR"
fi

if [ ! -f "$ZONE_CHANGER_DIR/$ZONE_SCRIPT" ]; then
    printf 'Unable to locate %s inside %s.\n' "$ZONE_SCRIPT" "$ZONE_CHANGER_DIR" >&2
    printf 'Re-run with POE_SYNC_FORCE=1 if you need to refresh the install.\n' >&2
    exit 1
fi

printf 'Launching PoE zone changer from %s...\n' "$ZONE_CHANGER_DIR"
exec python3 "$ZONE_CHANGER_DIR/$ZONE_SCRIPT"
