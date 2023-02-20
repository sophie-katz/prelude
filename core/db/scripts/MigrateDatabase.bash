#!/bin/bash

# MIT License
#
# Copyright (c) 2023 Sophie Katz
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

set -e

function usage() {
    echo "usage: $0 <database> <subcommand>" >&2
    echo >&2
    echo "OPTIONS" >&2
    echo "  database - The name of the Postgres database against which to run migration." >&2
    echo "  subcommand - Subcommand passed to the 'migrate' command of 'sea-orm-cli'" >&2
}

DATABASE="$1"
SUBCOMMAND="$2"

if [ -z "$DATABASE" ]; then
    echo "error: database parameter is missing" >&2
    echo >&2
    usage
    exit 1
elif [ -z "$SUBCOMMAND" ]; then
    echo "error: subcommand parameter is missing" >&2
    echo >&2
    usage
    exit 1
fi

cd /app/core/db && sea-orm-cli migrate -u "postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${DATABASE}" "${SUBCOMMAND}"
