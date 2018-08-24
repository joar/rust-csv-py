from typing import Optional


class CSVError(Exception):
    pass


class UTF8Error(CSVError):
    pass


class UnequalLengthsError(CSVError):
    pass
