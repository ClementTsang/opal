#!/bin/python3

import sqlite3
import sys

if len(sys.argv) < 3:
    print("Please include the cmudict directory and the name of the SQLite database you wish to write to.")
else:
    cmudict = sys.argv[1]
    sqlite_db = sys.argv[2]
