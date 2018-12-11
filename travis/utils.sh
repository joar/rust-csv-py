#!/bin/bash

# BEGIN UTILS
# Output utilities - DO NOT EDIT - Automatically inserted by travis/utils.sh
# ==============================================================================
function green { printf "\x1b[32m%s\x1b[0m\n" "$@" >&2; }
function red { printf "\x1b[31m%s\x1b[0m\n" "$@" >&2; }
function yellow { printf "\x1b[33m%s\x1b[0m\n" "$@" >&2; }
function message { echo "$@" >&2; } # like echo, but prints to stderr
# END UTILS

# Insert utility functions into file
# ==============================================================================

USAGE="$(cat <<USAGE
Usage: $0 TARGET_FILE

- This file contains the canonical "UTILS".
- Run this file update the TARGET_FILE's utils with the UTILS from this file.
USAGE
)"

TARGET_FILE="${1:?"$(printf '%s\n%s' "Missing TARGET_FILE argument" "$USAGE")"}"

TEMP_FILE="$(mktemp --suffix "$(basename "$TARGET_FILE")")"
# Set the same permissions for the temporary file as TARGET_FILE
chmod --reference="$TARGET_FILE" "$TEMP_FILE"

function clean {
  # Remove TEMP_FILE if it still exists
  if test -f "$TEMP_FILE"; then
    rm "$TEMP_FILE"
  fi
}

trap clean EXIT

function update_utils {
  local new_utils_path
  new_utils_path="${1:?}"
  sed -E -e '/^# BEGIN UTILS/,/^# END UTILS/{r '"$new_utils_path" -e 'd}'
}

update_utils \
  <(sed -n -E '/^# BEGIN UTILS/,/^# END UTILS/p' < "$0") \
  < "$TARGET_FILE" > "$TEMP_FILE"
# sed -E -e '/^# BEGIN UTILS/,/^# END UTILS/{r '<(sed -n -E '/^# BEGIN UTILS/,/^# END UTILS/p' < "$0") -e 'd}' "$TARGET_FILE" > "$TEMP_FILE"
if ! git diff --no-index "$TARGET_FILE" "$TEMP_FILE"; then
  read -r -p "Uptade utils in $TARGET_FILE? [y/n]: " prompt_response
  if ! [[ "$prompt_response" =~ ^y$ ]]; then
    red "Not updating"
    exit 42
  else
    green "Updated utils in $TARGET_FILE"
    mv "$TEMP_FILE" "$TARGET_FILE"
  fi
else
  yellow "Utils in $TARGET_FILE are already up to date"
fi

