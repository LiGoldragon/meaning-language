#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
printf '| Schema | IAST | English |\n|---|---|---|\n'
jq -r '.entries[] | "| \(.id) | \(.iast // "—") | \(.english) |"' "$root/glossary.json"
