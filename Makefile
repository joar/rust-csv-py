# Prefix to use when running python code.
PY_RUN ?= pipenv run
# Triggers the --release flag on or off when setup.py is building the rust
# extension module.
RUST_EXTENSION_DEBUG ?= True
RUSTFLAGS ?= target-cpu=native

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
		develop

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


