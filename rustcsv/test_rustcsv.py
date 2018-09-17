import contextlib
import io
import logging
import tempfile
from typing import Iterable, Union

import pytest
import rustcsv.error
from rustcsv import CSVReader, CSVWriter

_log = logging.getLogger(__name__)


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


def test_from_path_not_found():
    with pytest.raises(
        FileNotFoundError,
        message=(
            "First argument interpreted as path, but the path does not exist."
        ),
    ):
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
            marks=pytest.mark.xfail(
                raises=rustcsv.error.UnequalLengthsError, strict=True
            ),
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


@contextlib.contextmanager
def byte_records(records: Union[bytes, Iterable[bytes]]):
    with tempfile.NamedTemporaryFile("wb") as writable_fd:
        if isinstance(records, bytes):
            records = [records]
        for record in records:
            writable_fd.write(record + b"\n")
        writable_fd.flush()
        yield open(writable_fd.name, "rb")


def test_raises_utf8error():
    with byte_records(
        [
            b"valid UTF-8,invalid UTF-8,",
            b"valid: \xf0\x9f\x90\x8d,invalid: \xa0\xa1,",
        ]
    ) as fd:
        with pytest.raises(rustcsv.error.UTF8Error, message="") as exc_info:
            list(CSVReader(fd))

        utf8_error = exc_info.value  # type: rustcsv.error.UTF8Error
        assert utf8_error.position is not None
        assert utf8_error.position == rustcsv.error.Position(
            byte=27, line=2, record=1
        )


def test_writer_invalid_row_type():
    pass


@pytest.mark.parametrize(
    [
        "terminator",
        "double_quote",
        "quote_style",
        "escape",
        "records",
        "expected",
    ],
    [
        (
            b"\n",
            True,
            "necessary",
            b"\\",
            [("hello", "world")],
            b"hello,world\n",
        ),
        (
            b"\n",
            True,
            "necessary",
            b"\\",
            [('quoted"', "world")],
            b'"quoted""",world\n',
        ),
        (
            b"\n",
            False,
            "necessary",
            b"\\",
            [('escaped quote"',)],
            b'"escaped quote\\""\n',
        ),
        (
            b"\n",
            False,
            "necessary",
            b"\\",
            [("must,be,quoted",)],
            b'"must,be,quoted"\n',
        ),
        (
            b"\n",
            True,
            "necessary",
            b"\\",
            [("must,be,quoted",)],
            b'"must,be,quoted"\n',
        ),
        (
            b"\n",
            True,
            "always",
            b"\\",
            [("always", "1", "quoted")],
            b'"always","1","quoted"\n',
        ),
    ],
    ids=repr,
)
def test_writer(
    terminator: bytes,
    double_quote: bool,
    quote_style: str,
    escape: bytes,
    records: Iterable[Iterable[str]],
    expected: bytes,
):
    with tempfile.NamedTemporaryFile("wb") as fd:
        writer = CSVWriter(
            fd,
            terminator=terminator,
            double_quote=double_quote,
            quote_style=quote_style,
            escape=escape,
        )
        for row in records:
            writer.writerow(row)

        writer.flush()
        r_fd = open(fd.name, "rb")
        result = r_fd.read()
        assert result == expected


def test_writer_invalid_args():
    fd = io.BytesIO()
    with pytest.raises(ValueError):
        CSVWriter(fd, quote_style="invalid")
