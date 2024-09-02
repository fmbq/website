#!/bin/sh
set -eu

sqlite3 sqlite.db "INSERT INTO user (id, email) VALUES ('$(uuidgen)', '$1');"
