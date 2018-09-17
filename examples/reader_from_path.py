import tempfile
from rustcsv import CSVReader

# Create a temporary file to put our CSV content in,
# automatically delete it once we're done.
with tempfile.NamedTemporaryFile(mode="w") as writable_fd:
    writable_fd.write(
        """\
spam1,spam2,spam3
spam4,spam5,spam6
"""
    )
    writable_fd.flush()

    for row_number, row in enumerate(CSVReader(writable_fd.name), start=1):
        print(
            "row #{row_number}: {row}".format(row_number=row_number, row=row)
        )

# Prints:
# row #1: ("spam1", "spam2", "spam3")
# row #2: ("spam4", "spam5", "spam6")
