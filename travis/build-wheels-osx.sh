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

skipped_versions() {
    local ENABLED_VERSIONS
    ENABLED_VERSIONS=("$@")
    declare -a CIBW_VERSIONS=(cp27 cp34 cp35 cp36 cp37)
    for VERSION in "${CIBW_VERSIONS[@]}"; do
        if ! grep "$VERSION" <<<"${ENABLED_VERSIONS[@]}" &> /dev/null; then
            echo "$VERSION"
        fi
    done
}


build_wheels() {
    WHEELHOUSE="${WHEELHOUSE:-"wheelhouse"}"
    declare -a ENABLED_VERSIONS=("${@}")

    install_rust

    pip install -U cibuildwheel
    export CIBW_BEFORE_BUILD="make requirements-files && pip install -r requirements.txt && pip install -r dev-requirements.txt"
    export CIBW_TEST_COMMAND="py.test --pyargs rustcsv"
    CIBW_SKIP="$(skipped_versions "${ENABLED_VERSIONS[@]}")"
    export CIBW_SKIP
    cibuildwheel --output-dir "$WHEELHOUSE"
}

build_wheels "$@"
