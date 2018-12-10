from __future__ import absolute_import

from typing import Union, BinaryIO

# Import the Rust extension module
from ._rustcsv import CSVReader as _RustCSVReader, CSVWriter, __build__

try:
    from ._version import version
except ImportError:
    version = "UNKNOWN"

__all__ = ["CSVReader", "CSVWriter", "__build__", "version"]

CSVReader = _RustCSVReader
# Can't subclass for better docstrings: https://github.com/PyO3/pyo3/issues/220
# class CSVReader(_RustCSVReader):
#     def __new__(
#         cls,
#         source: Union[BinaryIO, str],
#         delimiter: bytes = b",",
#         terminator: bytes = b"\n",
#     ):
#         """
#
#         Parameters
#         ----------
#         source
#             :any:`binary file` or string to read CSV from.
#         delimiter
#             Byte to use as CSV field delimiter
#         terminator
#             Byte to use as CSV record terminator
#
#         Returns
#         -------
#         CSVReader
#         """
#         self = super(CSVReader, cls).__new__(
#             cls, path_or_fd=source, delimiter=delimiter, terminator=terminator
#         )
#
#         return self
