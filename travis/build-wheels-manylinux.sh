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

# Install a system package required by our library
# yum install -y openssl-devel

build_wheels()  {
    "$TRAVIS_DIR"/install-rust.sh
    export PATH="$HOME/.cargo/bin:$PATH"

    # Parameters
    WHEELHOUSE="${WHEELHOUSE:-"/io/wheelhouse"}"
    local SELECTED_VERSIONS=("$@")

    PYBINS="$(list_pybins "${SELECTED_VERSIONS[@]}")"
    if test -z "$PYBINS"; then
        red "No python versions found for ${SELECTED_VERSIONS[*]}"
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

list_pybins() {
    local bin
    for py in "$@"; do
        for bin in /opt/python/"${py}"*/bin; do
            echo "$bin"
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

(set -e -x;
build_wheels "$@")
