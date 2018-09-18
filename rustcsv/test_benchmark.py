import csv
import enum
import io
import logging
import string
import tempfile
from functools import partial
from typing import BinaryIO, Iterable

import attr
import hypothesis.strategies
import pytest
from _pytest.fixtures import FixtureRequest
from pytest_benchmark.fixture import BenchmarkFixture
from rustcsv import CSVReader

_log = logging.getLogger(__name__)


class Parser(enum.Enum):
    STDLIB = enum.auto()
    RUST = enum.auto()
    RUST_NO_PY_READER = enum.auto()


class FileStorage(enum.Enum):
    DISK = enum.auto()
    MEMORY = enum.auto()


class ColumnType(enum.Enum):
    INTEGER = enum.auto()
    UNICODE = enum.auto()
    UNICODE_LONG = enum.auto()
    ASCII = enum.auto()
    ASCII_LONG = enum.auto()


def get_reader(impl: Parser, path: str) -> Iterable[Iterable[str]]:
    if impl is Parser.STDLIB:
        return csv.reader(open(path, "r"))
    elif impl is Parser.RUST:
        return CSVReader(open(path, "rb"))
    elif impl is Parser.RUST_NO_PY_READER:
        return CSVReader(path)
    else:
        raise ValueError("Invalid impl: {impl}".format(impl=impl))


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
        raise ValueError("Invalid impl: {impl}".format(impl=impl))


@attr.s()
class CSVFixture:
    path = attr.ib(type=str)  # type: str
    rows = attr.ib(type=int)  # type: int
    column_type = attr.ib(type=ColumnType)  # type: ColumnType


skip_if_not_full = pytest.mark.skipif(
    "not bool(int(os.environ.get('BENCHMARK_FULL', 0)))"
)


@pytest.fixture(
    scope="module",
    params=[
        1000,
        10 * 1000,
        pytest.param(100 * 1000, marks=skip_if_not_full),
        pytest.param(1000 * 1000, marks=skip_if_not_full),
    ],
)
def fx_benchmark_row_count(request: FixtureRequest):
    """
    Number of CSV rows to generate
    """
    _log.debug("num_rows=%r", request.param)
    return request.param


@pytest.fixture(scope="module", params=ColumnType.__members__.values())
def fx_benchmark_column_type(request: FixtureRequest):
    _log.debug("column_type=%r", request.param)
    return request.param


@pytest.fixture(scope="module")
def fx_csv_file(fx_benchmark_row_count, fx_benchmark_column_type: ColumnType):
    """
    Generates a CSV file
    """
    _log.info(
        "Generating CSV file: rows=%r, column_type=%r",
        fx_benchmark_row_count,
        fx_benchmark_column_type,
    )
    column_type = fx_benchmark_column_type
    with tempfile.NamedTemporaryFile(mode="wb") as fd:
        wrapped_fd = wrap_fd(Parser.STDLIB, fd, write=True)
        writer = csv.writer(wrapped_fd)
        for i in range(fx_benchmark_row_count):
            if column_type is ColumnType.INTEGER:
                row = (i % 1000,) * 10
            elif column_type is ColumnType.UNICODE:
                row = ("aoeu", "xyz", "æ" * (i % 42))
            elif column_type is ColumnType.UNICODE_LONG:
                row = [hypothesis.strategies.text(average_size=100)]
            elif column_type is ColumnType.ASCII:
                row = ("aoeu", "xyz", "a" * (i % 42))
            elif column_type is ColumnType.ASCII_LONG:
                row = [
                    hypothesis.strategies.text(
                        alphabet=string.printable, average_size=100
                    )
                    for i in range(5)
                ]
            else:
                raise ValueError(
                    "Invalid column_type: {column_type}".format(
                        column_type=column_type
                    )
                )

            writer.writerow([str(i) for i in row])

        wrapped_fd.flush()

        yield CSVFixture(
            path=fd.name,
            rows=fx_benchmark_row_count,
            column_type=fx_benchmark_column_type,
        )


def read_csv(impl: Parser, path: str):
    i = 0
    reader = get_reader(impl, path)
    for i, row in enumerate(reader, start=1):
        pass
    return i


@pytest.mark.benchmark(min_rounds=10)
@pytest.mark.parametrize(
    "impl", [Parser.RUST, Parser.RUST_NO_PY_READER, Parser.STDLIB]
)
def test_benchmark_read(
    benchmark: BenchmarkFixture, impl, fx_csv_file: CSVFixture
):
    benchmark.group = "test_benchmark_read-{column_type}-{rows}".format(
        column_type=fx_csv_file.column_type, rows=fx_csv_file.rows
    )
    args = (fx_csv_file.path,)
    read_row_count = benchmark(partial(read_csv, impl), *args)

    assert read_row_count == fx_csv_file.rows
