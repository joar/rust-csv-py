#!/bin/bash
set -e -x

make clean build-wheels-manylinux publish-wheelhouse-test
