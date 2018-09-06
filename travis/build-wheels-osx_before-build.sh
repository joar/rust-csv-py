#!/bin/bash

make requirements-files \
    && pip install -r requirements.txt \
    && pip install -r dev-requirements.txt
