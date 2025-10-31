#!/bin/sh -e

# Prevent execution if this script was only partially downloaded
{
RC='\033[0m'
RED='\033[0;31m'

check() {
    exit_code=$1
    message=$2

    if [ "$exit_code" -ne 0 ]; then
        printf '%sERROR: %s%s\n' "$RED" "$message" "$RC"
        exit 1
    fi

    unset exit_code
    unset message
}

cleanup() {
    [ -n "$TMPDIR" ] && [ -d "$TMPDIR" ] && rm -rf "$TMPDIR"
}

if ! command -v git >/dev/null 2>&1; then
    printf '%sERROR: git is required by the bootstrapper.%s\n' "$RED" "$RC"
    printf '%sInstall git or rework bootstrap.sh to use pre-built releases (see linutil for an example).%s\n' "$RED" "$RC"
    exit 1
fi

REPO_URL=${WPSI_UTILS_REPO_URL:-"https://github.com/Wpsi1337/wpsi_utils.git"}
BRANCH=${WPSI_UTILS_BRANCH:-"master"}
ENTRYPOINT=${WPSI_UTILS_ENTRYPOINT:-"bin/wpsi_utils.sh"}

TMPDIR=$(mktemp -d)
check $? "Creating temporary directory"
trap cleanup EXIT INT TERM

git clone --depth 1 --branch "$BRANCH" "$REPO_URL" "$TMPDIR/repo" >/dev/null 2>&1
check $? "Cloning $REPO_URL@$BRANCH"

RUN_TARGET="$TMPDIR/repo/$ENTRYPOINT"
if [ ! -f "$RUN_TARGET" ]; then
    printf '%sERROR: Entrypoint %s missing in cloned repo.%s\n' "$RED" "$ENTRYPOINT" "$RC"
    exit 1
fi

if [ ! -x "$RUN_TARGET" ]; then
    chmod +x "$RUN_TARGET" 2>/dev/null || true
fi

if [ ! -x "$RUN_TARGET" ]; then
    printf '%sERROR: Entrypoint %s is not executable.%s\n' "$RED" "$ENTRYPOINT" "$RC"
    exit 1
fi

"$RUN_TARGET" "$@"
check $? "Executing $ENTRYPOINT"
} # End of wrapping
