from typing import Optional

import attr


@attr.s(slots=True, auto_attribs=True, cmp=True, frozen=True)
class Position:
    byte: int
    line: int
    record: int


class CSVError(Exception):
    pass


@attr.s(auto_attribs=True, cmp=True, frozen=True)
class UTF8Error(CSVError):
    message: str
    position: Optional[Position] = None


@attr.s(auto_attribs=True, cmp=True, frozen=True)
class UnequalLengthsError(CSVError):
    message: str
    position: Optional[Position] = None
