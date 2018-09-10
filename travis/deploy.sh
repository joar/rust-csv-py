#!/bin/bash
set -e -x

which twine || pip install -U twine

make clean build-wheels-manylinux publish-wheelhouse-test
