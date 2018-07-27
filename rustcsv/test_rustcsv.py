import csv
import logging
import tempfile
from functools import partial
from pathlib import Path

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

from rustcsv import CSVReader


@pytest.fixture(autouse=True, scope="session")
def configure_logging():
    import colorlog

    handler = colorlog.StreamHandler()
    handler.setFormatter(
        colorlog.ColoredFormatter(
            "%(log_color)s%(levelname)s:%(name)s:%(message)s"
        )
    )

    logger = logging.getLogger()
    logger.addHandler(handler)
    logger.setLevel(logging.DEBUG)


@pytest.fixture()
def applelike_csv_bytes() -> bytes:
    return b"x\x01y\x01z\x02" b"a\x01b\x01c\n\n\x02"


@pytest.fixture()
def applelike_csv_file(applelike_csv_bytes) -> Path:
    with tempfile.NamedTemporaryFile("w+b") as writable_csv_fd:
        writable_csv_fd.write(applelike_csv_bytes)
        yield Path(writable_csv_fd.name)


@pytest.mark.parametrize(
    "csv_content, expected",
    [
        pytest.param(
            b"x\x01y\x01z\x02" b"a\x01b\x01c\n\n\x02",
            [("x", "y", "z"), ("a", "b", "c\n\n")],
        )
    ],
    ids=repr,
)
def test_reader(csv_content, expected):
    with tempfile.NamedTemporaryFile("w+b", dir="/dev/shm/") as writable_csv_fd:
        writable_csv_fd.write(csv_content)
        writable_csv_fd.flush()
        result = list(
            CSVReader(
                writable_csv_fd.name, delimiter=b"\x01", terminator=b"\x02"
            )
        )
        assert result == expected


def process_row(row):
    a, b, c, *rest = row
    assert int(a) + int(b) == int(c)


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
        fd.write(f"{i},{i * 2},{i * 3},{'x' * i}".encode() + b"\n")

    fd.flush()


@pytest.mark.parametrize(
    "implementation", [impl_rust, impl_stdlib]
)
def test_read_csv_10_000(benchmark: BenchmarkFixture, implementation):
    rounds = 20
    row_count = 10_000
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


@pytest.mark.skip()
def test_repr(applelike_csv_file: Path):
    reader = CSVReader(str(applelike_csv_file))
    reader_repr = repr(reader)
    assert reader_repr == ""
