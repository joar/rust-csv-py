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


class Parser(enum.Enum):
    STDLIB = enum.auto()
    RUST = enum.auto()


class FileStorage(enum.Enum):
    DISK = enum.auto()
    MEMORY = enum.auto()


class ColumnType(enum.Enum):
    INTEGERS = enum.auto()
    UNICODE = enum.auto()
    UNICODE_LONG = enum.auto()
    ASCII = enum.auto()


def get_reader(impl: Parser, path: str) -> Iterable[Iterable[str]]:
    if impl is Parser.STDLIB:
        return csv.reader(open(path, "r"))
    elif impl is Parser.RUST:
        return CSVReader(open(path, "rb"))
    else:
        raise ValueError(f"Invalid impl: {impl}")


def wrap_fd(impl: Parser, fd: BinaryIO, write: bool = False):
    if impl is Parser.RUST:
        return fd
    elif impl is Parser.STDLIB:
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
    wrapped_fd = wrap_fd(Parser.STDLIB, fd, write=True)
    writer = csv.writer(wrapped_fd)
    for i in range(rows):
        if column_type is ColumnType.INTEGERS:
            row = (i % 1000,) * 10
        elif column_type is ColumnType.UNICODE:
            row = ("aoeu", "xyz", "æ" * (i % 42))
        elif column_type is ColumnType.UNICODE_LONG:
            row = ("aoeu", "xyz", "æ" * (i % 42)) * 10
        elif column_type is ColumnType.ASCII:
            row = ("aoeu", "xyz", "a" * (i % 42))
        else:
            raise ValueError(f"Invalid column_type: {column_type}")

        writer.writerow([str(i) for i in row])

    fd.flush()


def read_csv(impl: Parser, path: str):
    i = 0
    reader = get_reader(impl, path)
    for i, row in enumerate(reader, start=1):
        pass
    return i


@pytest.mark.benchmark(min_rounds=10)
@pytest.mark.parametrize("impl", [Parser.RUST, Parser.STDLIB])
@pytest.mark.parametrize("column_type", ColumnType.__members__.values())
@pytest.mark.parametrize(
    "row_count",
    [
        1_000,
        10_000,
        100_000,
        pytest.param(
            1_000_000,
            marks=pytest.mark.skipif(
                "not bool(int(os.environ.get('BENCHMARK_LARGE', 0))) "
            ),
        ),
    ],
)
def test_benchmark_read(
    benchmark: BenchmarkFixture, impl, column_type: ColumnType, row_count: int
):
    benchmark.group = f"test_benchmark_read-{column_type}-{row_count}"
    with tempfile.NamedTemporaryFile("wb") as writable_csv_fd:
        args = (writable_csv_fd.name,)
        generate_csv(writable_csv_fd, row_count, column_type)
        read_row_count = benchmark.pedantic(
            partial(read_csv, impl), args, iterations=10
        )

    assert read_row_count == row_count
