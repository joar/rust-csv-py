=========
CSVReader
=========

.. autoclass:: rustcsv.CSVReader

    .. automethod:: __new__

.. py:class:: rustcsv.CSVReader(path_or_file, delimiter=None, terminator=None)

    Creates a new CSVReader instance

    Arguments:

    ``path_or_file`` (:class:`str` or :any:`binary file`)

        -   A :class:`str` path to a file.
        -   A :any:`binary file` object, 
            e.g. :class:`io.BytesIO` or ``open(path, "rb")``.

    ``delimiter`` (Optional :class:`bytes` of length 1)
        The CSV field delimiter.
        Defaults to ``b","`` if ``None``.
    ``terminator`` (Optional :class:`bytes` of length 1)
        The CSV record terminator.
        Defaults to ``b"\n"`` if ``None``.
