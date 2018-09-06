#!/bin/bash

pip install cibuildwheel

make clean build-wheels-osx publish-test
