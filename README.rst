.. |rust-csv| replace:: ``rust-csv``
.. _rust-csv: https://github.com/BurntSushi/rust-csv

.. |pyo3| replace:: ``PyO3``
.. _pyo3: https://github.com/PyO3/pyo3

################################################################################
                     |rust-csv|_ + |PyO3|_ = Fast CSV Parsing
################################################################################

BIG DISCLAIMER
================================================================================

-   This is not a production-ready library.
-   I'm not a production-ready Rust programmer.
-   Python 3's ``csv`` stdlib module is pretty %#!& fast too. At least compared
    to this package.


Benchmark
================================================================================

1.  Figure out the dependencies and install them, in my case it's:

    -   cargo + rust as required by |rust-csv|_ and |pyo3|_.
    -   Python 3.6

2.

    .. code-block:: console

        $ make benchmark
