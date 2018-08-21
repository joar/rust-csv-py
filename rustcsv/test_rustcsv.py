import io
import logging
import tempfile
from pathlib import Path

import pytest
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


def test_str_argument_instead_of_file_like():
    with pytest.raises(AttributeError, message="str has no attribute 'read'"):
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
def test_reader(csv_content, expected):
    with tempfile.NamedTemporaryFile("w+b") as writable_csv_fd:
        writable_csv_fd.write(csv_content)
        writable_csv_fd.flush()
        csv_reader = CSVReader(
            open(writable_csv_fd.name, "rb"),
            delimiter=b"\x01",
            terminator=b"\x02",
        )
        result = list(csv_reader)
        assert result == expected


if __name__ == "__main__":
    fd = io.BytesIO()
    csv_content = b"x\x01y\x01z\x02" b"a\x01b\x01c\n\n\x02"
    fd.write(csv_content)
    fd.seek(0)
    print(list(CSVReader(fd, delimiter=b"\x01", terminator=b"\x02")))

    expected = [("x", "y", "z"), ("a", "b", "c\n\n")]
    with tempfile.NamedTemporaryFile("w+b") as writable_csv_fd:
        writable_csv_fd.write(csv_content)
        writable_csv_fd.flush()
        result = list(
            CSVReader(
                open(writable_csv_fd.name, "rb"),
                delimiter=b"\x01",
                terminator=b"\x02",
            )
        )
        assert result == expected
