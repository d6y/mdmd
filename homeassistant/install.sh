#!/bin/sh
# Copies the mdmd app, with the Rust source, into a Home Assistant
# addons folder (by default the Samba share mounted on a Mac).
#
# Usage: homeassistant/install.sh [addons-dir]
set -eu

ADDONS_DIR="${1:-/Volumes/addons}"
REPO="$(cd "$(dirname "$0")/.." && pwd)"
TARGET="$ADDONS_DIR/mdmd"

if [ ! -d "$ADDONS_DIR" ]; then
  echo "No addons folder at $ADDONS_DIR" >&2
  echo "In Finder use Go > Connect to Server (Cmd-K) with smb://homeassistant.local/addons" >&2
  exit 1
fi

mkdir -p "$TARGET"
rsync -a --delete \
  --exclude .DS_Store \
  "$REPO/homeassistant/mdmd/" \
  "$REPO/Cargo.toml" \
  "$REPO/Cargo.lock" \
  "$REPO/src" \
  "$TARGET/"

echo "Installed app into $TARGET"
