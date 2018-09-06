#!/bin/bash
# script directory, e.g. "<git-repo>/travis/"
TRAVIS_DIR="$(dirname "${BASH_SOURCE[0]}")"
# shellcheck source=travis/_output_helpers.sh
source "$TRAVIS_DIR/_output_helpers.sh"

# Install a system package required by our library
# yum install -y openssl-devel

build_wheels()  {
    bash "$TRAVIS_DIR"/install_rust.sh

    # Parameters
    WHEELHOUSE="${WHEELHOUSE:-"/io/wheelhouse"}"
    local SELECTED_VERSIONS=("$@")

    PYBINS="$(list_pybins "$@")"
    if test -z "$PYBINS"; then
        red "No python versions found for ${SELECTED_VERSIONS[@]}"
    fi

    # Compile wheels
    # ==============
    for PYBIN in $PYBINS; do
        green "Building wheel for $("${PYBIN}/python" --version)"
        build_wheel "$PYBIN"
    done

    # Bundle external shared libraries into the wheels
    # ================================================
    for whl in wheelhouse/*.whl; do
        auditwheel repair "$whl" -w "$WHEELHOUSE"
    done

    # Install packages and test
    # =========================
    for PYBIN in $PYBINS; do
        "${PYBIN}/pip" install rustcsv --no-index -f "$WHEELHOUSE"
        (cd "$HOME"; "${PYBIN}/py.test" --pyargs rustcsv)
    done
}

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

list_pybins() {
    local bin
    for py in "$@"; do
        for bin in /opt/python/"${py}"*/bin; do
            # Ignore 3.4 and 3.5 since we use f-strings
            if ! grep -E "cp34|cp35" <<<"$bin" &> dev/null; then
                echo "$bin"
            else
                echo "Skipping $bin" >&2
            fi
        done
    done
}

export_paths() {
    local PYBIN
    PYBIN="${1:?}"
}

build_wheel() {
    local PYBIN
    PYBIN="${1:?}"
    local PYTHON_LIB
    local RUST_LIB_PATH
    RUST_LIB_PATH="$HOME/rust/lib"
    PYTHON_LIB="$("${PYBIN}/python" -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))")"
    PKG_CONFIG_PATH="${PYTHON_LIB}/pkgconfig"
    declare -a env_vars=()
    # Set up paths for cargo & PyO3
    # -----------------------------
    # Tell build.rs where "python" is
    env_vars+=('PYTHON_SYS_EXECUTABLE='"$PYBIN/python")
    env_vars+=('PYTHON_LIB='"$PYTHON_LIB")
    env_vars+=('PKG_CONFIG_PATH='"$PKG_CONFIG_PATH")
    env_vars+=('LIBRARY_PATH='"$LIBRARY_PATH:$PYTHON_LIB")
    env_vars+=('LD_LIBRARY_PATH='"$PYTHON_LIB:$RUST_LIB_PATH:$LD_LIBRARY_PATH")
    "${PYBIN}/pip" install -r /io/dev-requirements.txt
    env "${env_vars[@]}" "${PYBIN}/pip" wheel /io/ -w wheelhouse/
}

if [[ "${BASH_SOURCE[0]}" = "${0}" ]]; then
    set -e -x
    build_wheels "$@"
else
    echo "Script was sourced, not executing build_wheels" >&2
fi
