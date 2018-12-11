#!/bin/bash
# script directory, e.g. "<git-repo>/travis/"
TRAVIS_DIR="$(dirname "${BASH_SOURCE[0]}")"
# shellcheck source=travis/_output_helpers.sh
source "$TRAVIS_DIR/_output_helpers.sh"

# Parameters
RUSTCSV_RUST_VERSION="${RUSTCSV_RUST_VERSION:-"nightly"}"

install_rust() {
    # install rust + cargo
    # ============================
    CARGO_BIN=$HOME/.cargo/bin
    if ! test -d "$CARGO_BIN"; then
        green "Installing rust + cargo, version $RUSTCSV_RUST_VERSION"
        curl https://sh.rustup.rs -sSf \
            | sh -s -- -y --default-toolchain "$RUSTCSV_RUST_VERSION"
    fi
    if ! grep "$CARGO_BIN" <<<"$PATH" &> /dev/null; then
        green "Adding $CARGO_BIN to \$PATH"
        export PATH="$CARGO_BIN:$PATH"
    fi
}


if [[ "${BASH_SOURCE[0]}" = "${0}" ]]; then
    red "The script should be sourced so that it's able to export \$PATH." \
        "e.g. 'source ${BASH_SOURCE[*]}' will install rust and re-export \$PATH"
    exit 1
else
    install_rust "$@"
fi
