#!/bin/bash

pip install cibuildwheel

make clean build-osx-wheel publish-test
