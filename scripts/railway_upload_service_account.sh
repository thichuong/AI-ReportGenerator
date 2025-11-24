#!/usr/bin/env bash
set -euo pipefail

# Upload service_account.json to Railway as GOOGLE_SERVICE_ACCOUNT_JSON environment variable.
# Usage: ./scripts/railway_upload_service_account.sh [--dry-run]
# Requirements: railway CLI must be logged in and linked to a project (railway link).

DRY_RUN=0
if [[ ${1-} == "--dry-run" ]]; then
  DRY_RUN=1
fi

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

SERVICE_ACCOUNT_FILE="service_account.json"
ENV_VAR_NAME="GOOGLE_SERVICE_ACCOUNT_JSON"

if [[ ! -f "$SERVICE_ACCOUNT_FILE" ]]; then
  echo "Error: $SERVICE_ACCOUNT_FILE not found in project root"
  exit 1
fi

echo "Found $SERVICE_ACCOUNT_FILE"
echo "Will upload as: $ENV_VAR_NAME"
echo "Dry run: $DRY_RUN"
echo

# Read the JSON content
json_content=$(cat "$SERVICE_ACCOUNT_FILE")

# Validate JSON format
if ! echo "$json_content" | python3 -m json.tool > /dev/null 2>&1; then
  echo "Error: $SERVICE_ACCOUNT_FILE is not valid JSON"
  exit 1
fi

echo "JSON validated successfully"

if [[ $DRY_RUN -eq 1 ]]; then
  echo "DRY RUN: railway variables --set \"$ENV_VAR_NAME=<json_content>\" --skip-deploys"
  echo
  echo "Preview (first 100 chars):"
  echo "${json_content:0:100}..."
  exit 0
fi

# Upload to Railway
# The JSON content may contain special characters, pass it safely
railway variables --set "$ENV_VAR_NAME=$json_content" --skip-deploys

if [[ $? -eq 0 ]]; then
  echo "Successfully uploaded $ENV_VAR_NAME to Railway"
else
  echo "Failed to upload $ENV_VAR_NAME" >&2
  exit 1
fi

echo
echo "Done. Verify with: railway variables --json | jq '.${ENV_VAR_NAME}'"
