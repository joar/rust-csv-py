#!/bin/bash
# Source this file in order to use these methods
green() {
    printf "\x1b[32m%s\x1b[0m\n" "$@" >&2
}

red() {
    printf "\x1b[31m%s\x1b[0m\n" "$@" >&2
}

yellow() {
    printf "\x1b[33m%s\x1b[0m\n" "$@" >&2
}

if [[ "${BASH_SOURCE[0]}" = "${0}" ]]; then
    red "This script is intended for use in 'source ${0}'"
    exit 1
fi
