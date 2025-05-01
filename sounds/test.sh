#!/usr/bin/env sh
set -u

LOG_FILE="duplicate_mp3_cleanup.log"

#-----------------------------
# Logging functions
#-----------------------------
log_info() {
  msg=$*
  ts=$(date '+%Y-%m-%d %H:%M:%S')
  echo "$ts [INFO]  $msg" | tee -a "$LOG_FILE"
}

log_error() {
  msg=$*
  ts=$(date '+%Y-%m-%d %H:%M:%S')
  echo "$ts [ERROR] $msg" | tee -a "$LOG_FILE" >&2
}

#-----------------------------
# Preconditions
#-----------------------------

# Can we write the log?
if ! touch "$LOG_FILE" 2>/dev/null; then
  echo "Cannot write to log file '$LOG_FILE'." >&2
  exit 1
fi

# Is b3sum available?
if ! command -v b3sum >/dev/null 2>&1; then
  log_error "b3sum not found. Please install the Blake3 toolkit."
  exit 1
fi

# Create a temporary index for seen checksums
# Fallback to a file in /tmp if mktemp is unavailable
INDEX_FILE=$(mktemp 2>/dev/null) || \
  INDEX_FILE="/tmp/mp3_dupe_index_$$.txt"
# Clean up on exit/INT/TERM
trap 'rm -f -- "$INDEX_FILE"' EXIT INT TERM

#-----------------------------
# Are there any .mp3 files?
#-----------------------------
found_mp3=false
for f in *.mp3; do
  if [ -e "$f" ]; then
    found_mp3=true
    break
  fi
done

if [ "$found_mp3" != "true" ]; then
  log_info "No .mp3 files found. Nothing to do."
  exit 0
fi

#-----------------------------
# Main loop: checksum & delete
#-----------------------------
for file in *.mp3; do
  # skip non-regular files
  if [ ! -f "$file" ]; then
    log_error "Skipping non-regular file: $file"
    continue
  fi

  # compute Blake3 checksum
  if ! hash_line=$(b3sum "$file" 2>/dev/null); then
    log_error "Failed to compute checksum for '$file'"
    continue
  fi
  # extract just the hash (first token)
  hash=${hash_line%% *}

  # have we seen this hash before?
  if grep -q "^$hash " "$INDEX_FILE"; then
    # grab the original filename (rest of the line after the first space)
    orig=$(grep "^$hash " "$INDEX_FILE" | head -n1 | cut -d' ' -f2-)
    if rm -f -- "$file"; then
      log_info "Deleted duplicate '$file' (matches '$orig')"
    else
      log_error "Failed to delete '$file'"
    fi
  else
    # record this new hash→file mapping
    if ! printf '%s %s\n' "$hash" "$file" >>"$INDEX_FILE"; then
      log_error "Failed to write to index file '$INDEX_FILE'"
      exit 1
    fi
    log_info "Keeping '$file' (hash=$hash)"
  fi
done
