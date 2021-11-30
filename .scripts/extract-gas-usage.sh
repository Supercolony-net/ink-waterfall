#!/bin/bash

# Extracts the gas usage from the `ink-waterfall` log.
#
# Usage:
#   ./extract-gas-usage.sh <ink-example-name>

set -eu
set -o pipefail

EXAMPLE=$($1)

USAGE=$(cat /tmp/waterfall.log |
  grep "example: " $1 ", " |
  egrep --only-matching "estimated gas for transaction is [0-9]*" |
  egrep --only-matching "[0-9]*" |
  awk '{s+=$1} END {print s}') || exit $?

echo "$EXAMPLE, $USAGE"
