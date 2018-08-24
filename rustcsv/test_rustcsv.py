import io
import logging
import tempfile
from pathlib import Path

import pytest
from rustcsv import CSVReader
from rustcsv.error import UnequalLengthsError


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


def test_str_argument_instead_of_file_like():
    with pytest.raises(TypeError, message="str has no attribute 'read'"):
        CSVReader("does-not-exist")


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
def test_delimiter_and_terminator(csv_content, expected):
    with tempfile.NamedTemporaryFile("wb") as writable_csv_fd:
        writable_csv_fd.write(csv_content)
        writable_csv_fd.flush()
        csv_reader = CSVReader(
            open(writable_csv_fd.name, "rb"),
            delimiter=b"\x01",
            terminator=b"\x02",
        )
        result = list(csv_reader)
        assert result == expected


@pytest.mark.parametrize(
    "csv_content, expected",
    [
        pytest.param(b"a,b\n" b"1,2", [("a", "b"), ("1", "2")]),
        pytest.param(
            b"a,b\n" b"1,2,3",
            [("a", "b"), ("1", "2")],
            marks=pytest.mark.xfail(raises=UnequalLengthsError, strict=True),
        ),
    ],
    ids=repr,
)
def test_reader(csv_content, expected):
    buf = io.BytesIO(csv_content)
    csv_reader = CSVReader(buf)
    result = list(csv_reader)
    assert result == expected


def test_text_io_error():
    with tempfile.NamedTemporaryFile("wb") as writable_fd:
        writable_fd.write(b"a,b,c\n1,2,3\n")
        writable_fd.flush()
        with pytest.raises(OSError):
            # Try passing in a text-mode file-like
            list(CSVReader(open(writable_fd.name)))
