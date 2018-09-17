.. |rust-csv| replace:: ``rust-csv``
.. _rust-csv: https://github.com/BurntSushi/rust-csv

.. |pyo3| replace:: ``PyO3``
.. _pyo3: https://github.com/PyO3/pyo3

.. |csv| replace:: ``csv``
.. _csv: https://docs.python.org/3/library/csv.html

.. |travis-badge| image:: https://travis-ci.com/joar/rust-csv-py.svg?branch=master
.. _travis-badge: https://travis-ci.com/joar/rust-csv-py

.. _`Travis CI project`: https://travis-ci.com/joar/rust-csv-py

################################################################################
|rust-csv|_ + |PyO3|_ = Not much slower than |csv|_ :tada:
################################################################################

|travis-badge|_

.. contents:: Contents
    :backlinks: none
    :local:

================================================================================
BIG DISCLAIMER
================================================================================

-   This is not a production-ready library.
-   I'm not a production-ready Rust programmer.
-   Either Python 3's |csv|_ stdlib module is pretty %#!& fast or my Rust code
    is %#!& slow.

================================================================================
Installation
================================================================================

.. code-block:: sh

    pip install rustcsv

================================================================================
Examples
================================================================================

--------------------------------------------------------------------------------
CSVReader instance from path
--------------------------------------------------------------------------------

`<examples/reader_from_path.py>`_:

.. code-block:: python

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


--------------------------------------------------------------------------------
CSVReader instance from a binary file object
--------------------------------------------------------------------------------

`<examples/reader_from_file_object.py>`_:

.. code-block:: python

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

        readable_fd = open(writable_fd.name, "rb")

        for row_number, row in enumerate(CSVReader(readable_fd), start=1):
            print(
                "row #{row_number}: {row}".format(row_number=row_number, row=row)
            )

    # Prints:
    # row #1: ("spam1", "spam2", "spam3")
    # row #2: ("spam4", "spam5", "spam6")


================================================================================
Development
================================================================================

--------------------------------------------------------------------------------
Development Installation
--------------------------------------------------------------------------------

Install and build the extension locally from e.g. a git checkout.

Requirements
================================================================================

-   `Pipenv <http://pipenv.org/>`_.
-   Python 3.6.
-   Rust and Cargo nightly (1.30 as of now) - `<https://rustup.rs/>`_.

Install Python dependencies
================================================================================

.. code-block:: sh

    pipenv install --dev

Build the ``rustcsv._rustcsv`` extension
================================================================================

Either

1.  Using the "debug" cargo profile, or

    .. code-block:: sh

        make develop-debug

2.  Using the "release" cargo profile

    .. code-block:: sh

        make develop-release

Run tests
================================================================================

.. code-block:: sh

    make test


Run benchmarks
================================================================================

.. code-block:: sh

    make benchmark

Note: ``make benchmark`` will always build the extension using the "release"
cargo profile.

================================================================================
Benchmarks
================================================================================

Benchmarks are executed as the last step in the `Travis CI project`_.

You can also run it yourself, see `Development`_ and `Run benchmarks`_.

================================================================================
References
================================================================================

-   `<https://github.com/python/cpython/blob/master/Modules/_csv.c>`_
-   `<https://pyo3.rs/v0.4.1/class.html>`_
