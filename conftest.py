import logging

import colorlog
import pytest


@pytest.fixture(scope="session", autouse=True)
def configure_logging():
    handler = logging.StreamHandler()
    handler.setFormatter(
        colorlog.ColoredFormatter(
            "%(log_color)s%(asctime)s%(levelname)8s %(name)s %(funcName)s\n"
            "%(message)s",
            log_colors={
                "DEBUG": "blue",
                "INFO": "green",
                "WARNING": "yellow",
                "ERROR": "red",
                "CRITICAL": "red,bg_white",
            },
        )
    )
    logging.root.handlers = []
    logging.root.setLevel(logging.DEBUG)
    logging.root.addHandler(handler)
