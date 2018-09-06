#!/bin/bash

install_rust() {
    # install rust + cargo nightly
    # ============================
    export RUST_VERSION=nightly
    CARGO_BIN=$HOME/.cargo/bin
    if ! test -d "$CARGO_BIN"; then
        green "Installing rust + cargo"
        curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $RUST_VERSION
    fi
    if ! grep "$CARGO_BIN" <<<"$PATH" &> /dev/null; then
        green "Addigng $CARGO_BIN to \$PATH"
        export PATH="$CARGO_BIN:$PATH"
    fi
}

filter_versions() {
    local CIBW_VERSIONS=("cp27 cp34 cp35 cp36 cp37")
}


build_wheels() {
    WHEELHOUSE="${WHEELHOUSE:-"wheelhouse"}"

    install_rust
    pip install -U cibuildwheel
    export CIBW_BEFORE_BUILD="pip install -r requirements.txt && pip install -r dev-requirements.txt"
    export CIBW_TEST_COMMAND="py.test --pyargs rustcsv"
    cibuildwheel --output-dir "$WHEELHOUSE"
}

build_wheels "$@"
