RUST_EXTENSION_DEBUG ?= True
RUSTFLAGS ?= target-cpu=native
PY_RUN ?= pipenv run
PYTEST_OPTS ?= -vvl --benchmark-skip

.PHONY: default
default:
	# Nothing

.PHONY: develop
develop:
	$(PY_RUN) python setup.py develop

.PHONY: build-debug
develop-debug:
	env RUST_EXTENSION_DEBUG=True \
		make develop

.PHONY: develop-release
develop-release:
	env RUST_EXTENSION_DEBUG=False \
		make develop

.PHONY: test
test:
	$(PY_RUN) pytest $(PYTEST_OPTS)

.PHONY: black
black:
	$(PY_RUN) black ./rustcsv

.PHONY: isort
isort:
	$(PY_RUN) isort -y

.PHONY: benchmark
benchmark: | develop-release
	$(PY_RUN) pytest \
		-vv \
		--showlocals \
		--benchmark-timer time.process_time \
		$$(: --benchmark-group-by 'func') \
		--benchmark-histogram \
		--benchmark-autosave \
		--benchmark-only


