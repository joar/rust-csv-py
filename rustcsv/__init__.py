from __future__ import absolute_import

import logging

from ._rustcsv import CSVReader as _CSVReader, CSVWriter as _CSVWriter

__all__ = ["CSVReader", "CSVWriter"]


CSVReader = _CSVReader
CSVWriter = _CSVWriter
