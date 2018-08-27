import io
import json

from rustcsv import CSVReader

fd = io.BytesIO(b"foo,bar,baz")

reader = CSVReader(fd)

print(len(list(reader)))

# scoped()

with open(".pytest_cache/v/cache/lastfailed") as fd2:
    print(json.load(fd2))
