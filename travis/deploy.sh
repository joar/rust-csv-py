#!/bin/bash
set -e -x

make clean build-manylinux-wheels publish-test
