#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "[run_server] Root: $ROOT"
echo "[run_server] Starting server..."
cargo run -p server
