#!/bin/sh
set -eu

sqlite3 -csv -header sqlite.db 'SELECT * FROM user'
