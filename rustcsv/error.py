from typing import Optional

import attr


@attr.s(slots=True, cmp=True, frozen=True)
class Position:
    byte = attr.ib(type=int)  # type: int
    line = attr.ib(type=int)  # type: int
    record = attr.ib(type=int)  # type: int


class CSVError(Exception):
    pass


@attr.s(cmp=True, frozen=True)
class UTF8Error(CSVError):
    message = attr.ib(type=str)  # type: str
    position = attr.ib(
        None, type=Optional[Position]
    )  # type: Optional[Position]


@attr.s(cmp=True, frozen=True)
class UnequalLengthsError(CSVError):
    message = attr.ib(type=str)  # type: str
    position = attr.ib(
        None, type=Optional[Position]
    )  # type: Optional[Position]
