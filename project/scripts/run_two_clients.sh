#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# WSLg: force X11 backend (XWayland) to avoid Wayland broken pipe
export DISPLAY=:0
export WINIT_UNIX_BACKEND=x11
unset WAYLAND_DISPLAY

cargo run -p client &
sleep 0.5
cargo run -p client &
wait
