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

# NOTE: This script is very similar to docker/postgres-initdb.d/CreateDatabases.bash

PGPASSWORD="$POSTGRES_PASSWORD" psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" --host "$POSTGRES_HOST" <<-EOSQL
    DROP DATABASE IF EXISTS prelude_dev;

    DROP DATABASE IF EXISTS prelude_unit;

    CREATE DATABASE prelude_dev;
    GRANT ALL PRIVILEGES ON DATABASE prelude_dev TO $POSTGRES_USER;

    CREATE DATABASE prelude_unit;
    GRANT ALL PRIVILEGES ON DATABASE prelude_unit TO $POSTGRES_USER;
EOSQL
