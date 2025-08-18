#!/usr/bin/env bash
set -x
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
cd "${ROOT}" || exit 1

ensure npx tailwindcss -i ./input.css -o ./tailwind.css
ensure npx tailwindcss -i ./extension-input.css -o ./extension.css
ensure npx wrangler r2 object put assets/tailwind.css --file tailwind.css --remote
ensure npx wrangler r2 object put assets/extension.css --file extension.css --remote
