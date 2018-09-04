#!/bin/bash
set -e -x

which twine || pip install -U twine

make clean build-manylinux-wheels publish-test
