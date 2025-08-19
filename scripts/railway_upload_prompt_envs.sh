#!/usr/bin/env bash
set -euo pipefail

# Upload prompt_envs/.env.* files to Railway as environment variables.
# Usage: ./scripts/railway_upload_prompt_envs.sh [--dry-run]
# Requirements: railway CLI must be logged in and linked to a project (railway link).

DRY_RUN=0
if [[ ${1-} == "--dry-run" ]]; then
  DRY_RUN=1
fi

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

shopt -s nullglob
FILES=(prompt_envs/.env.*)
if [[ ${#FILES[@]} -eq 0 ]]; then
  echo "No files found in prompt_envs/.env.*"
  exit 0
fi

echo "Found ${#FILES[@]} files to process. Dry run: $DRY_RUN"

echo
for f in "${FILES[@]}"; do
  echo "----"
  echo "File: $f"
  # Read the whole file content and split at the first '='
  # Assumes each file contains a single assignment like KEY="...multiline..."
  raw=$(cat "$f")
  # Extract key (before first '=')
  key=${raw%%=*}
  # Extract value (everything after first '=')
  value=${raw#*=}
  # Remove surrounding leading/trailing quotes if present
  if [[ ${value:0:1} == '"' ]]; then
    # remove first character
    value=${value:1}
    # remove trailing quote if present
    # Use parameter expansion to strip the last char only if it's a quote
    if [[ ${value: -1} == '"' ]]; then
      value=${value:0:-1}
    fi
  fi

  echo "Key: $key"
  # Do not print value

  if [[ $DRY_RUN -eq 1 ]]; then
    echo "DRY RUN: railway variables --set \"$key\" --value-from-file <temp-file> --skip-deploys"
    echo "Value length: ${#value} characters"
    continue
  fi

  # Set variable on Railway. Use --skip-deploys to avoid triggering deploys.
  # The value may contain newlines; pass it safely via a temporary file.
  temp_file=$(mktemp)
  echo -n "$value" > "$temp_file"
  
  railway variables --set "$key" --value-from-file "$temp_file" --skip-deploys
  
  if [[ $? -eq 0 ]]; then
    echo "✅ Uploaded: $key"
  else
    echo "❌ Failed to upload: $key" >&2
  fi
  
  rm -f "$temp_file"

done

echo
echo "Done. Verify with: railway variables --json" 
