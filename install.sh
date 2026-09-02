#!/bin/sh
set -e

REPO="criggil/LocalAgentToolchain"
INSTALL_DIR="${LAT_INSTALL_DIR:-$HOME/.local/bin}"

# Detect OS and Architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
  darwin)
    case "$ARCH" in
      arm64|aarch64) TARGET="aarch64-apple-darwin" ;;
      x86_64)        TARGET="x86_64-apple-darwin" ;;
      *) echo "Unsupported macOS architecture: $ARCH" >&2; exit 1 ;;
    esac
    ;;
  linux)
    case "$ARCH" in
      x86_64|amd64) TARGET="x86_64-unknown-linux-gnu" ;;
      *) echo "Unsupported Linux architecture: $ARCH" >&2; exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported operating system: $OS" >&2
    exit 1
    ;;
esac

TARBALL="local-agent-toolchain-${TARGET}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${TARBALL}"

printf "\n\033[1;34m==>\033[0m Installing \033[1mLocal Agent Toolchain (LAT)\033[0m...\n"
printf "    Platform: %s\n" "$TARGET"
printf "    Install destination: %s\n" "$INSTALL_DIR"

mkdir -p "$INSTALL_DIR"

# Download tarball into temporary directory
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

printf "\033[1;34m==>\033[0m Downloading latest release from GitHub...\n"
curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/$TARBALL"

printf "\033[1;34m==>\033[0m Extracting binaries...\n"
tar -xzf "$TMP_DIR/$TARBALL" -C "$TMP_DIR"

# Install binaries
cp -f "$TMP_DIR/task" "$INSTALL_DIR/task"
cp -f "$TMP_DIR/note" "$INSTALL_DIR/note"
cp -f "$TMP_DIR/skill" "$INSTALL_DIR/skill"

chmod +x "$INSTALL_DIR/task" "$INSTALL_DIR/note" "$INSTALL_DIR/skill"

printf "\n\033[1;32m✓\033[0m Successfully installed:\n"
printf "  • %s/task\n" "$INSTALL_DIR"
printf "  • %s/note\n" "$INSTALL_DIR"
printf "  • %s/skill\n" "$INSTALL_DIR"

# Check if INSTALL_DIR is in PATH
case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *)
    printf "\n\033[1;33mWarning:\033[0m %s is not in your PATH.\n" "$INSTALL_DIR"
    printf "Add the following line to your ~/.zshrc or ~/.bashrc:\n\n"
    printf "  export PATH=\"%s:\$PATH\"\n\n" "$INSTALL_DIR"
    ;;
esac

printf "\nRun \033[1mtask --help\033[0m, \033[1mnote --help\033[0m, or \033[1mskill --help\033[0m to get started!\n\n"
