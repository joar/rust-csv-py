import click
from rustcsv import CSVReader


@click.group()
def cli():
    pass


@cli.command()
@click.argument("file", type=click.File(mode="rb"))
def read(file):
    for row in CSVReader(file):
        print(row)


if __name__ == "__main__":
    cli()
