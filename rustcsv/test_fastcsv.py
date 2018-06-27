import tempfile

import pytest

from rustcsv import CSVReader


@pytest.mark.parametrize(
    "csv_content, expected",
    [
        pytest.param(
            b"x\x01y\x01z\x02" b"a\x01b\x01c\n\n\x02",
            [["x", "y", "z"], ["a", "b", "c\n\n"]],
        )
    ],
    ids=repr,
)
def test_reader(csv_content, expected):
    with tempfile.NamedTemporaryFile("w+b") as writable_csv_fd:
        writable_csv_fd.write(csv_content)
        writable_csv_fd.flush()
        result = list(CSVReader(writable_csv_fd.name))
        assert result == expected
