RUST_EXTENSION_DEBUG ?= True
PY_RUN ?= pipenv run
PYTEST_OPTS ?= -vvl --benchmark-skip
GEOLITE_EN_CSV_PATH = res/csv/geolite-city-en.csv

.PHONY: default
default:
	# Nothing

.PHONY: develop
develop:
	$(PY_RUN) python setup.py develop

.PHONY: build-debug
develop-debug:
	$(PY_RUN) env RUST_EXTENSION_DEBUG=True \
		python setup.py develop

.PHONY: build-release
develop-release:
	$(PY_RUN) env RUST_EXTENSION_DEBUG=False \
		python setup.py develop

.PHONY: benchmark
benchmark: $(GEOLITE_EN_CSV_PATH) | build-release
	$(PY_RUN) pytest \
		-vv \
		--showlocals \
		--benchmark-timer time.process_time \
		--benchmark-group-by 'func' \
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

# Benchmarking data
# ------------------------------------------------------------------------------

GEOLITE_CITY_CSV_ZIP_URL = http://geolite.maxmind.com/download/geoip/database/GeoLite2-City-CSV.zip
GEOLITE_CITY_CSV_ZIP = res/csv/GeoLite2-City-CSV.zip
GEOLITE_EN_CSV_ZIP_PATH_FILE = $(GEOLITE_CITY_CSV_ZIP).en-csv.path.txt
GEOLITE_EN_CSV_ZIP_NAME = GeoLite2-City-Locations-en.csv

.PHONY: clean-csv-downloaded
clean-csv-downloaded:
	rm -f '$(GEOLITE_CITY_CSV_ZIP)'

.PHONY: clean-csv-derived
clean-csv-derived:
	find res/csv -type f \
		| grep -v '$(GEOLITE_CITY_CSV_ZIP)$$' \
		| xargs -r rm -v

$(GEOLITE_CITY_CSV_ZIP):
	curl \
		--location \
		'$(GEOLITE_CITY_CSV_ZIP_URL)' \
		> $@

$(GEOLITE_EN_CSV_ZIP_PATH_FILE): $(GEOLITE_CITY_CSV_ZIP)
	# Extract the path that GeoLite2-City-Locations-en.csv has inside the zip
	# archive since the file is inside a date-stamped directory
	# (e.g. GeoLite2-City-CSV_20180703/GeoLite2-City-Locations-en.csv).
	unzip -lqq '$<' \
		| awk '{print $$4}' \
		| grep '$(GEOLITE_EN_CSV_ZIP_NAME)' \
		> $@

$(GEOLITE_EN_CSV_PATH): $(GEOLITE_EN_CSV_ZIP_PATH_FILE)
	unzip -p \
		'$(GEOLITE_CITY_CSV_ZIP)' \
		"$$(cat $<)" \
		> $@
