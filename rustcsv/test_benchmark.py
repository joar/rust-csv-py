import csv
import tempfile
from functools import partial

import pytest
from pytest_benchmark.fixture import BenchmarkFixture
from rustcsv import CSVReader


def process_row(row):
    a, b, c, *rest = row
    assert int(a) + int(b) == int(c)


def impl_read_rust(path):
    for row in CSVReader(str(path)):
        yield row


def impl_read_stdlib(path):
    for row in csv.reader(open(path)):
        yield row


def count_rows(reader_impl, path):
    i = 0
    for i, row in enumerate(reader_impl(path), start=1):
        pass
    return i


@pytest.mark.parametrize("impl_read", [impl_read_rust, impl_read_stdlib])
def test_read_geolite_city_en_csv(benchmark: BenchmarkFixture, impl_read):
    result = benchmark(
        partial(count_rows, impl_read, "res/csv/geolite-city-en.csv")
    )
    assert result is not None


def impl_rust(path):
    i = 0
    for i, row in enumerate(CSVReader(str(path)), start=1):
        process_row(row)
    return i


def impl_stdlib(path):
    i = 0
    for i, row in enumerate(csv.reader(open(path)), start=1):
        process_row(row)
    return i


def write_large_csv(fd, rows=10_000):
    for i in range(rows):
        fd.write(f"{i},{i * 2},{i * 3},a".encode() + b"\n")

    fd.flush()


@pytest.mark.parametrize("implementation", [impl_rust, impl_stdlib])
def test_read_csv_100_000(benchmark: BenchmarkFixture, implementation):
    rounds = 20
    row_count = 100_000
    with tempfile.NamedTemporaryFile("wb") as writable_csv_fd:
        args = (writable_csv_fd.name,)
        # write_large_csv(writable_csv_fd, row_count)
        # read_row_count = benchmark(implementation, *args)
        read_row_count = benchmark.pedantic(
            implementation,
            args,
            setup=partial(write_large_csv, writable_csv_fd, row_count),
            rounds=rounds,
        )

    assert read_row_count == row_count * rounds
