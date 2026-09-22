#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
exec bash ./build-v7.sh "$@"
