import csv
import enum
import io
import logging
import tempfile
from functools import partial
from typing import Iterable, BinaryIO

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

from rustcsv import CSVReader

_log = logging.getLogger(__name__)


class Implementation(enum.Enum):
    STDLIB = enum.auto()
    RUST = enum.auto()


class FileStorage(enum.Enum):
    DISK = enum.auto()
    MEMORY = enum.auto()


class ColumnType(enum.Enum):
    INTEGERS = enum.auto()
    UNICODE = enum.auto()
    ASCII = enum.auto()


def get_reader(impl: Implementation, path: str) -> Iterable[Iterable[str]]:
    if impl is Implementation.STDLIB:
        return csv.reader(open(path, "r"))
    elif impl is Implementation.RUST:
        return CSVReader(open(path, "rb"))
    else:
        raise ValueError(f"Invalid impl: {impl}")


def wrap_fd(impl: Implementation, fd: BinaryIO, write: bool = False):
    if impl is Implementation.RUST:
        return fd
    elif impl is Implementation.STDLIB:
        # The standard library must have a text-mode file-like.
        if fd.name:
            mode = "w" if write else "r"
            _log.info("Opening %r in mode %r", fd.name, mode)
            return open(fd.name, mode)
        else:
            return io.TextIOWrapper(fd)
    else:
        raise ValueError(f"Invalid impl: {impl}")


def generate_csv(fd: BinaryIO, rows: int, column_type: ColumnType):
    wrapped_fd = wrap_fd(Implementation.STDLIB, fd, write=True)
    writer = csv.writer(wrapped_fd)
    for i in range(rows):
        if column_type is ColumnType.INTEGERS:
            row = (i, i * 2, i * 3)
        elif column_type is ColumnType.UNICODE:
            row = ("aoeu", "xyz", "æ" * (i % 42))
        elif column_type is ColumnType.ASCII:
            row = ("aoeu", "xyz", "a" * (i % 42))
        else:
            raise ValueError(f"Invalid column_type: {column_type}")

        writer.writerow([str(i) for i in row])

    fd.flush()


def write_large_csv(fd, rows=10_000):
    for i in range(rows):
        fd.write(f"{i},{i * 2},{i * 3},{'x' * (i // 2)}".encode() + b"\n")

    fd.flush()


def read_csv(impl: Implementation, path: str):
    i = 0
    reader = get_reader(impl, path)
    for i, row in enumerate(reader, start=1):
        pass
    return i


@pytest.mark.benchmark(min_rounds=10)
@pytest.mark.parametrize("impl", [Implementation.RUST, Implementation.STDLIB])
@pytest.mark.parametrize("column_type", ColumnType.__members__.values())
@pytest.mark.parametrize("row_count", [10_000, 100_000, 1_000_000])
def test_benchmark_read(
    benchmark: BenchmarkFixture, impl, column_type: ColumnType, row_count: int
):
    benchmark.group = f"test_benchmark_read-{column_type}-{row_count}"
    with tempfile.NamedTemporaryFile("wb") as writable_csv_fd:
        args = (writable_csv_fd.name,)
        generate_csv(writable_csv_fd, row_count, column_type)
        # read_row_count = benchmark(implementation, *args)
        read_row_count = benchmark(partial(read_csv, impl), *args)

    assert read_row_count == row_count
