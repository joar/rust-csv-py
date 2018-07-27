RUST_EXTENSION_DEBUG ?= True
PY_RUN ?= pipenv run
PYTEST_OPTS ?= -vvl --benchmark-skip

.PHONY: develop
develop:
	$(PY_RUN) python setup.py develop

.PHONY: build-release
build-release:
	$(PY_RUN) env RUST_EXTENSION_DEBUG=False python setup.py develop

.PHONY: benchmark
benchmark: | build-release
	$(PY_RUN) pytest \
		-vv \
		--showlocals \
		--benchmark-histogram \
		--benchmark-autosave \
		--benchmark-only

.PHONY: test
test:
	$(PY_RUN) pytest $(PYTEST_OPTS)

.PHONY: black
black:
	$(PY_RUN) black ./rustcsv

.PHONY: isort
isort:
	$(PY_RUN) isort -y
