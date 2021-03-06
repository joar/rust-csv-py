# Prefix to use when running python code.
PY_RUN ?= pipenv run
# Triggers the --release flag on or off when setup.py is building the rust
# extension module.
RUSTCSV_BUILD_DEBUG ?= True
RUSTCSV_BUILD_NATIVE ?= True
RUSTCSV_RUST_VERSION ?= nightly
MANYLINUX_IMAGE ?= quay.io/pypa/manylinux1_x86_64
WHEEL_PYTHON_VERSIONS ?= cp36 cp37
WHEELHOUSE = wheelhouse

.PHONY: default
default:
	# Do nothing and fail if no target specified
	false

# Development
# ===========

.PHONY: black
black:
	# Format python code
	$(PY_RUN) black ./rustcsv

.PHONY: isort
isort:
	# Sort imports in python code
	$(PY_RUN) isort -y

.PHONY: develop
develop:
	$(PY_RUN) python setup.py develop

.PHONY: develop-debug
develop-debug:
	make \
		RUSTCSV_BUILD_DEBUG=True \
		develop

.PHONY: develop-release
develop-release:
	make \
		RUSTCSV_BUILD_DEBUG=False \
		RUSTCSV_BUILD_NATIVE=True \
		develop

.PHONY: clean
clean: | setuptools-clean

.PHONY: setuptools-clean
setuptools-clean:
	$(PY_RUN) python setup.py clean

# Testing
# =======

# pytest options
PYTEST_OPTS ?= -vv --showlocals
# additional pytest options when running tests
PYTEST_TEST_OPTS ?=
# additional pytest options when running benchmarks
PYTEST_BENCHMARK_OPTS ?=

PYTEST_BENCHMARK_TIMER ?= time.process_time
PYTEST_BENCHMARK_SORT ?= fullname

.PHONY: test
test:
	# Run python tests
	$(PY_RUN) pytest \
		$(PYTEST_OPTS) \
		--benchmark-skip \
		$(PYTEST_TEST_OPTS)

.PHONY: test-example-scripts
test-example-scripts:
	$(PY_RUN) python examples/reader_from_path.py
	$(PY_RUN) python examples/reader_from_file_object.py

# Run heavy benchmarks, 1 = True, 0 = False
BENCHMARK_FULL ?= 0

.PHONY: benchmark
benchmark: | develop-release
	# Run benchmarks
	$(PY_RUN) pytest \
		$(PYTEST_OPTS) \
		--benchmark-only \
		--benchmark-timer $(PYTEST_BENCHMARK_TIMER) \
		--benchmark-sort $(PYTEST_BENCHMARK_SORT) \
		--benchmark-autosave

.PHONY: benchmark-full
benchmark-full:
	make BENCHMARK_FULL=1 benchmark

# Docs
# ====

.PHONY: docs
docs:
	make -C doc html

.PHONY: docs-apidoc
docs-apidoc:
	sphinx-apidoc -o doc/source/api/ -H "API" rustcsv 'test_*'

.PHONY: docs-autobuild
docs-autobuild:
	sphinx-autobuild -p 8001 doc/source/ doc/build/html/

# Release management
# ==================

.PHONY: build-release-sdist
build-release-sdist:
	$(PY_RUN) env \
		RUSTCSV_BUILD_DEBUG=False \
		RUSTCSV_BUILD_NATIVE=True \
		python setup.py \
		sdist

.PHONY: reqirements-files
requirements-files:
	# Generate reqirements file
	$(PY_RUN) pipenv lock --requirements > requirements.txt
	# Generate dev reqirements file
	$(PY_RUN) pipenv lock --requirements --dev > dev-requirements.txt

.PHONY: build-wheels-manylinux
build-wheels-manylinux: | requirements-files
	docker run --rm -it \
		-v $(shell pwd):/io \
		--env RUSTCSV_BUILD_DEBUG=$(RUSTCSV_BUILD_DEBUG) \
		--env RUSTCSV_BUILD_NATIVE=$(RUSTCSV_BUILD_NATIVE) \
		--env RUSTCSV_RUST_VERSION=$(RUSTCSV_RUST_VERSION) \
		--env WHEELHOUSE=/io/$(WHEELHOUSE) \
		$(MANYLINUX_IMAGE) \
		/io/travis/build-wheels-manylinux.sh $(WHEEL_PYTHON_VERSIONS)

.PHONY: build-sdist
build-sdist:
	$(PY_RUN) python setup.py sdist --dist-dir $(WHEELHOUSE)

.PHONY: build-wheels-osx
build-wheels-osx: | reqirements-files
	$(PY_RUN) env \
		WHEELHOUSE=$(WHEELHOUSE) \
		bash travis/build-wheels-osx.sh $(WHEEL_PYTHON_VERSIONS)

.PHONY: publish-wheelhouse
publish-wheelhouse:
	# Publish files in $(WHEELHOUSE) to PyPI
	$(PY_RUN) twine upload \
		--username rustcsv \
		$(WHEELHOUSE)/*

.PHONY: publish-wheelhouse-test
publish-wheelhouse-test:
	# Publish files in $(WHEELHOUSE) to Test PyPI:
	# https://packaging.python.org/guides/using-testpypi/
	$(PY_RUN) twine upload \
		--repository-url https://test.pypi.org/legacy/ \
		--username testrustcsv \
		$(WHEELHOUSE)/*
