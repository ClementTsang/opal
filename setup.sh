#!/bin/sh

# Pull submodule sources
git submodule update --init --recursive

# Create DBs
mkdir ./static/databases
touch ./static/databases/db.sqlite3
python3 ./scripts/cmudict/cmudict.py ./sources/cmudict ./static/databases/db.sqlite3
sqlite3 static/databases/db.sqlite3 < scripts/optimize_sql.sql
cp ./sources/cmudict/README ./static/databases/
cp ./sources/cmudict/LICENSE ./static/databases/
