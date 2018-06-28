.. |rust-csv| replace:: ``rust-csv``
.. _rust-csv: https://github.com/BurntSushi/rust-csv

.. |pyo3| replace:: ``PyO3``
.. _pyo3: https://github.com/PyO3/pyo3

################################################################################
|rust-csv|_ + |PyO3|_ = Not much slower than Python 3's ``csv`` :tada:
################################################################################

BIG DISCLAIMER
================================================================================

-   This is not a production-ready library.
-   I'm not a production-ready Rust programmer.
-   Python 3's ``csv`` stdlib module is pretty %#!& fast.

Benchmark
================================================================================

1.  Figure out the dependencies and install them, in my case it's:

    -   cargo + rust as required by |rust-csv|_ and |pyo3|_.
    -   Python 3.6 and pipenv

2.

    .. code-block:: console

        $ make benchmark
        
**Spoiler:** It's about tied on my machine. Python 3's ``csv`` has the upper 
hand, I might have an ace up my sleeve if I figure out how to convert
``csv::StringRecord`` straight to ``pyo3::PyTuple`` instead of ``pyo3::PyList``.
