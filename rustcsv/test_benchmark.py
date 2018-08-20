import csv
import enum
import io
import tempfile
from functools import partial
from typing import Iterable, BinaryIO

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

from rustcsv import CSVReader


class ReaderImplementation(enum.Enum):
    STDLIB = enum.auto()
    RUST = enum.auto()


def get_reader(
    impl: ReaderImplementation, fd: BinaryIO
) -> Iterable[Iterable[bytes]]:
    if impl is ReaderImplementation.STDLIB:
        if fd.name:
            return csv.reader(open(fd.name))
        else:
            return csv.reader(io.TextIOWrapper(fd))
    if impl is ReaderImplementation.RUST:
        return CSVReader(fd)


def count_rows(reader_impl: ReaderImplementation, path):
    i = 0
    with open(path, "rb") as fd:
        for i, row in enumerate(get_reader(reader_impl, fd), start=1):
            pass
    return i


@pytest.mark.parametrize(
    "impl_read", [ReaderImplementation.RUST, ReaderImplementation.STDLIB]
)
def test_read_geolite_city_en_csv(benchmark: BenchmarkFixture, impl_read):
    result = benchmark(
        partial(count_rows, impl_read, "res/csv/geolite-city-en.csv")
    )
    assert result is not None


def write_large_csv(fd, rows=10_000):
    for i in range(rows):
        fd.write(f"{i},{i * 2},{i * 3},{'x' * (i // 2)}".encode() + b"\n")

    fd.flush()


def process_row(row):
    a, b, c, *rest = row
    assert int(a) + int(b) == int(c)


def process_rows(impl: ReaderImplementation, path: str):
    i = 0
    with open(path, "rb") as fd:
        reader = get_reader(impl, fd)
        for i, row in enumerate(reader, start=1):
            process_row(row)
    return i


@pytest.mark.parametrize(
    "implementation", [ReaderImplementation.RUST, ReaderImplementation.STDLIB]
)
def test_read_csv_10_000(benchmark: BenchmarkFixture, implementation):
    rounds = 20
    row_count = 10_000
    with tempfile.NamedTemporaryFile("wb") as writable_csv_fd:
        args = (writable_csv_fd.name,)
        # write_large_csv(writable_csv_fd, row_count)
        # read_row_count = benchmark(implementation, *args)
        read_row_count = benchmark.pedantic(
            partial(process_rows, implementation),
            args,
            setup=partial(write_large_csv, writable_csv_fd, row_count),
            rounds=rounds,
        )

    assert read_row_count == row_count * rounds
