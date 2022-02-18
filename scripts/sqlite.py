#!/bin/python3

import sqlite3
import sys

if len(sys.argv) < 2:
    print("Please include the name of the SQLite database you wish to write to.")
else:
    