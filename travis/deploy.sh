#!/bin/bash
set -e -x

pip install --user -U twine

make clean build-manylinux-wheels publish-test
