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

.PHONY: build-release
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

# Resource files
# ==============================================================================

RESOURCE_PATH ?= res/csv

$(RESOURCE_PATH)/.created:
	mkdir -p $(RESOURCE_PATH)
	touch $(RESOURCE_PATH)/.created

.PHONY: $(RESOURCE_PATH)
$(RESOURCE_PATH): $(RESOURCE_PATH)/.created

# GeoLite2 CSV
# ------------------------------------------------------------------------------

# Source URL for the zip
GEOLITE_CITY_CSV_ZIP_URL = http://geolite.maxmind.com/download/geoip/database/GeoLite2-City-CSV.zip
# Destination path for the downloaded zip
GEOLITE_CITY_CSV_ZIP = $(RESOURCE_PATH)/GeoLite2-City-CSV.zip
# Destination path for the "en" csv extracted from the zip
GEOLITE_EN_CSV_PATH = $(RESOURCE_PATH)/geolite-city-en.csv
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

$(GEOLITE_CITY_CSV_ZIP): $(RESOURCE_PATH)
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


# Depends on resources
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


