#!/bin/python3

import os
import sys

if len(sys.argv) <= 1:
    print("Please include the directory containing the .wasm files.")
else:
    TMP_FILE = "tmp_output"
    for file in os.listdir(os.fsencode(sys.argv[-1])):
        filename = os.fsdecode(file)
        target = os.path.join(sys.argv[-1], filename)

        if filename.endswith(".wasm"):
            if os.system("wasm-opt -Os -o {} {}".format(TMP_FILE, target)) == 0:
                try:
                    os.replace(TMP_FILE, target)
                    print("Successfully optimized a {}".format(target))
                except:
                    print("Error: Could not rename file")
                    sys.exit(42)
            else:
                print("Error: Failed to optimize {}".format(target))
                sys.exit(42)
        elif filename.endswith(".js"):
            if os.system("minify {} > {}".format(target, TMP_FILE)) == 0:
                try:
                    os.replace(TMP_FILE, target)
                    print("Successfully optimized a {}".format(target))
                except:
                    print("Error: Could not rename file")
                    sys.exit(42)
            else:
                print("Error: Failed to optimize {}".format(target))
                sys.exit(42)
