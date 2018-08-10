import io

from rustcsv import CSVReader

for i in range(0, 1):
    fd = io.BytesIO(b"foo,bar,baz")

    reader = CSVReader(fd)

    print(len(list(reader)))
