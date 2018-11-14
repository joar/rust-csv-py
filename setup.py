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
            "Could not parse environment variable {key}'s value {value} as "
            "bool ".format(key=key, value=value)
        )


RUSTCSV_BUILD_DEBUG = get_env_bool("RUSTCSV_BUILD_DEBUG", False)
RUSTCSV_BUILD_NATIVE = get_env_bool("RUSTCSV_BUILD_NATIVE", True)

try:
    from setuptools_rust import RustExtension, Binding
except ImportError:
    import subprocess

    errno = subprocess.call(
        [sys.executable, "-m", "pip", "install", "setuptools-rust"]
    )
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension, Binding


class PyTest(TestCommand):
    user_options = []

    def run(self):
        self.run_command("test_rust")

        import subprocess
        import sys

        errno = subprocess.call([sys.executable, "-m", "pytest", "tests"])
        raise SystemExit(errno)


setup_requires = ["setuptools-rust>=0.10.1", "setuptools_scm>=3.1.0"]
install_requires = ["attrs >= 18.2.0", "click >= 6.7"]
tests_require = install_requires + ["pytest", "pytest-benchmark"]

LONG_DESCRIPTION = None

try:
    LONG_DESCRIPTION = open("README.rst").read()
except Exception:
    pass

setup(
    name="rustcsv",
    use_scm_version=dict(write_to="rustcsv/_version.py"),
    author="Joar Wandborg",
    author_email="joar@wandborg.se",
    url="https://github.com/joar/rust-csv-py",
    long_description=LONG_DESCRIPTION,
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
            "rustcsv._rustcsv",
            "Cargo.toml",
            binding=Binding.PyO3,
            native=RUSTCSV_BUILD_NATIVE,
            debug=RUSTCSV_BUILD_DEBUG,
        )
    ],
    entry_points={"console_scripts": ["rustcsv=rustcsv.__main__:cli"]},
    tests_require=tests_require,
    setup_requires=setup_requires,
    install_requires=install_requires,
    include_package_data=True,
    zip_safe=False,
    cmdclass=dict(test=PyTest),
)
