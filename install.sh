#!/usr/bin/env bash

set -e

TARGET_NAME="giant-proxy"
INSTALL_DIR="${HOME}/.local/bin"
REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "🔧 Installing ${TARGET_NAME}..."

mkdir -p "$INSTALL_DIR"

# Use symlink instead of copying
ln -sf "${REPO_DIR}/${TARGET_NAME}" "${INSTALL_DIR}/${TARGET_NAME}"
chmod +x "${REPO_DIR}/${TARGET_NAME}"

echo "✅ Symlinked ${TARGET_NAME} to ${INSTALL_DIR}"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "⚠️  ${INSTALL_DIR} is not in your PATH"
    echo "   You may want to add this to your shell profile:"
    echo "   export PATH=\"${INSTALL_DIR}:\$PATH\""
else
    echo "   You can now run '${TARGET_NAME}' from anywhere."
fi
