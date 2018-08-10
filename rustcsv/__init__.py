from __future__ import absolute_import

import logging

from ._rustcsv import CSVReader as _CSVReader

__all__ = ["CSVReader"]


CSVReader = _CSVReader
# class CSVReader(_CSVReader):
#     pass
# def __init__(self, *args, **kwargs):
#     super(CSVReader, self).__init__()
