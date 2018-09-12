from __future__ import absolute_import

from .rustcsv import CSVReader, CSVWriter, __build__

try:
    from ._version import version
except ImportError:
    version = "UNKNOWN"

__all__ = ["CSVReader", "CSVWriter", "__build__", "version"]
