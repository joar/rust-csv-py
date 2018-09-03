# Prefix to use when running python code.
PY_RUN ?= pipenv run
# Triggers the --release flag on or off when setup.py is building the rust
# extension module.
RUST_EXTENSION_DEBUG ?= True
RUST_EXTENSION_NATIVE ?= False
MANYLINUX_IMAGE ?= quay.io/pypa/manylinux1_x86_64

.PHONY: default
default:
	# Nothing

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
		RUST_EXTENSION_DEBUG=True \
		develop

.PHONY: develop-release
develop-release:
	make \
		RUST_EXTENSION_DEBUG=False \
		RUST_EXTENSION_NATIVE=True \
		develop

# Release management
# ==============================================================================

.PHONY: build-release
build-release-sdist:
	$(PY_RUN) env \
		RUST_EXTENSION_DEBUG=False \
		RUST_EXTENSION_NATIVE=True \
		python setup.py \
		sdist

.PHONY: reqirements-files
requirements-files:
	# Generate reqirements file
	$(PY_RUN) pipenv lock --requirements > requirements.txt
	# Generate dev reqirements file
	$(PY_RUN) pipenv lock --requirements --dev > dev-requirements.txt

.PHONY: build-manylinux-wheels
build-manylinux-wheels: | requirements-files
	docker run --rm -it \
		-v $(shell pwd):/io \
		--env RUST_EXTENSION_DEBUG=$(RUST_EXTENSION_DEBUG) \
		--env RUST_EXTENSION_NATIVE=$(RUST_EXTENSION_NATIVE) \
		$(MANYLINUX_IMAGE) \
		/io/travis/build-wheels.sh

.PHONY: build-release-manylinux-wheels
build-release-manylinux-wheels:
	make \
		RUST_EXTENSION_DEBUG=False \
		RUST_EXTENSION_NATIVE=False \
		build-manylinux-wheels

.PHONY: publish-test
publish-test:
	# Publish wheels to Test PyPI:
	# https://packaging.python.org/guides/using-testpypi/
	$(PY_RUN) twine upload \
		--repository-url https://test.pypi.org/legacy/ \
		--username testrustcsv
		wheels/*

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
		--benchmark-histogram \
		--benchmark-autosave

.PHONY: benchmark-full
benchmark-full:
	make BENCHMARK_FULL=1 benchmark
