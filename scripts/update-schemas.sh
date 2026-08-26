#!/usr/bin/env bash
# Refresh the vendored SMAPI schemas.
#
# Review the diff before committing: these are validation rules, and upstream
# tightens them between game versions, so a refresh can start reporting errors
# in files that were previously clean.
#
# smapi.io serves them with CRLF and .gitattributes pins the tree to LF, so the
# line endings are converted here — otherwise every refresh would rewrite every
# line and bury the change that actually matters.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

for name in manifest content-patcher i18n; do
    target="$root/schemas/$name.json"
    curl -fsSL "https://smapi.io/schemas/$name.json" | tr -d '\r' > "$target"
    printf '%-22s %6s bytes\n' "$name.json" "$(wc -c < "$target" | tr -d ' ')"
done
