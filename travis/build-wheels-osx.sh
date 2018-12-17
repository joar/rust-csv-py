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

build_wheels() {
    set -x
    WHEELHOUSE="${WHEELHOUSE:-"wheelhouse"}"
    declare -a ENABLED_VERSIONS=("${@}")

    source "$TRAVIS_DIR"/install-rust.sh  # re-exports $PATH

    pip install -U cibuildwheel

    export CIBW_BEFORE_BUILD="bash -x ${TRAVIS_DIR}/build-wheels-osx_before-build.sh"
    export CIBW_TEST_COMMAND="py.test --pyargs rustcsv"
    CIBW_SKIP="$(skipped_versions "${ENABLED_VERSIONS[@]}")"
    export CIBW_SKIP
    cibuildwheel --output-dir "$WHEELHOUSE"
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

if [[ "${BASH_SOURCE[0]}" = "${0}" ]]; then
    set -e -x
    build_wheels "$@"
else
    echo "Script was sourced, not executing build_wheels"
fi
