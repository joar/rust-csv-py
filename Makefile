RUST_EXTENSION_DEBUG ?= True
PY_RUN ?= pipenv run

.PHONY: develop
develop:
	$(PY_RUN) python setup.py develop

.PHONY: benchmark
benchmark:
	# Don't give the standard library an unfair advantage
	$(PY_RUN) env RUST_EXTENSION_DEBUG=False python setup.py develop
	$(PY_RUN) pytest \
		-vv \
		--showlocals \
		--benchmark-histogram \
		--benchmark-autosave \
		--benchmark-only
