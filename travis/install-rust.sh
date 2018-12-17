#!/bin/bash
# script directory, e.g. "<git-repo>/travis/"
TRAVIS_DIR="$(dirname "${BASH_SOURCE[0]}")"

# BEGIN UTILS
# Output utilities - DO NOT EDIT - Automatically inserted by travis/utils.sh
# ==============================================================================
function green { printf "\x1b[32m%s\x1b[0m\n" "$@" >&2; }
function red { printf "\x1b[31m%s\x1b[0m\n" "$@" >&2; }
function yellow { printf "\x1b[33m%s\x1b[0m\n" "$@" >&2; }
function message { echo "$@" >&2; } # like echo, but prints to stderr
# END UTILS

# Parameters
RUSTCSV_RUST_VERSION="${RUSTCSV_RUST_VERSION:-"nightly"}"

install_rust() {
    # install rust + cargo
    # ============================
    CARGO_BIN=$HOME/.cargo/bin
    green "Installing rust + cargo, version $RUSTCSV_RUST_VERSION"
    curl https://sh.rustup.rs -sSf \
        | sh -s -- -y --default-toolchain "$RUSTCSV_RUST_VERSION"
    if ! grep "$CARGO_BIN" <<<"$PATH" &> /dev/null; then
        yellow "Warning: $CARGO_BIN is not in your \$PATH"
    fi
}

install_rust "$@"
