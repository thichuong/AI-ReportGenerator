#!/bin/bash
set -e

echo "Starting container: $(date)"
echo "Environment variables:"
env | sort

echo "Waiting briefly for dependent services..."
sleep 1

# Run the passed command
exec "$@"
