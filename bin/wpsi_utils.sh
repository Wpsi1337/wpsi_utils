#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PROJECT_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
SCRIPTS_DIR="$PROJECT_ROOT/scripts"
CONFIG_FILE="$PROJECT_ROOT/config/default.conf"

# Load optional configuration so scripts can pick up defaults.
if [ -f "$CONFIG_FILE" ]; then
    # shellcheck disable=SC1090
    . "$CONFIG_FILE"
fi

print_available() {
    printf '%s\n' "${DEFAULT_MESSAGE:-"wpsi_utils is ready. Pass a command to run a script."}"
    printf '%s\n' "Available scripts:"
    found=false
    for script in "$SCRIPTS_DIR"/*.sh; do
        [ -e "$script" ] || continue
        found=true
        name=$(basename "$script" .sh)
        printf '  - %s\n' "$name"
    done
    if [ "$found" = false ]; then
        printf '  (none yet — add shell scripts under %s)\n' "$SCRIPTS_DIR"
    fi
}

if [ $# -eq 0 ]; then
    print_available
    exit 0
fi

command=$1
shift || true
SCRIPT_PATH="$SCRIPTS_DIR/$command.sh"

if [ ! -f "$SCRIPT_PATH" ]; then
    printf 'Unknown command: %s\n' "$command" >&2
    print_available >&2
    exit 1
fi

if [ ! -x "$SCRIPT_PATH" ]; then
    chmod +x "$SCRIPT_PATH" 2>/dev/null || true
fi

exec "$SCRIPT_PATH" "$@"
