#!/usr/bin/env bash

set -e

TARGET_NAME="giant-proxy"
INSTALL_DIR="${HOME}/.local/bin"
REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "🔧 Installing ${TARGET_NAME}..."

# Ensure target dir exists
mkdir -p "$INSTALL_DIR"

# Copy the binary
cp "${REPO_DIR}/${TARGET_NAME}" "${INSTALL_DIR}/${TARGET_NAME}"
chmod +x "${INSTALL_DIR}/${TARGET_NAME}"

# Ensure ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  echo "⚠️  ~/.local/bin is not in your PATH"
  echo "   You may want to add this to your shell profile:"
  echo "   export PATH=\"$INSTALL_DIR:\$PATH\""
else
  echo "✅ ${TARGET_NAME} installed to ${INSTALL_DIR}"
  echo "   You can now run '${TARGET_NAME}' from anywhere."
fi
