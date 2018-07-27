import os
import sys

from setuptools import find_packages, setup
from setuptools.command.test import test as TestCommand


def get_env_bool(key, default=None):
    value = os.environ.get(key)
    if value is None:
        return default

    if value.lower() in ("true", "1"):
        return True
    elif value.lower() in ("false", "0", ""):
        return False
    else:
        raise ValueError(
            f"Could not parse environment variable {key}'s value {value} as "
            f"bool "
        )


RUST_EXTENSION_DEBUG = get_env_bool("RUST_EXTENSION_DEBUG")

try:
    from setuptools_rust import RustExtension
except ImportError:
    import subprocess

    errno = subprocess.call(
        [sys.executable, "-m", "pip", "install", "setuptools-rust"]
    )
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension


class PyTest(TestCommand):
    user_options = []

    def run(self):
        self.run_command("test_rust")

        import subprocess
        import sys

        errno = subprocess.call([sys.executable, "-m", "pytest", "tests"])
        raise SystemExit(errno)


setup_requires = ["setuptools-rust>=0.10.1"]
install_requires = []
tests_require = install_requires + ["pytest", "pytest-benchmark"]

setup(
    name="rustcsv",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
    ],
    packages=find_packages(),
    rust_extensions=[
        RustExtension(
            "rustcsv._rustcsv", "Cargo.toml", debug=RUST_EXTENSION_DEBUG
        )
    ],
    tests_require=tests_require,
    setup_requires=setup_requires,
    install_requires=install_requires,
    include_package_data=True,
    zip_safe=False,
    cmdclass=dict(test=PyTest),
)
