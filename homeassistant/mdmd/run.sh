#!/bin/sh
# Home Assistant app entry point.
# Reads app options, then runs mdmd once for every line received on stdin
# (sent by the hassio.addon_stdin action from an automation).
set -eu

OPTIONS="${OPTIONS:-/data/options.json}"

opt() { jq -r ".$1" "$OPTIONS"; }

export GITHUB_TOKEN="$(opt github_token)"
export GITHUB_REPO="$(opt github_repo)"
export GITHUB_BRANCH="$(opt github_branch)"
export NUM_POSTS="$(opt num_posts)"
export RUST_LOG="$(opt log_level)"

echo "mdmd: waiting for triggers"
while read -r _; do
  echo "mdmd: triggered"
  mdmd || echo "mdmd: failed (exit $?)"
done
