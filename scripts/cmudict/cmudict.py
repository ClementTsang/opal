#!/bin/python3

import sqlite3
import sys
import pandas as pd
import os
import re


def trim_parens(s: str):
    pos = s.find("(")
    if pos == -1:
        return s
    else:
        return s[0:pos]


def trim_numbers(s: str):
    m = re.search(r"\d", s)
    if m:
        pos = m.start()
        return s[0:pos]
    else:
        return s


def fix(phonemes: list[str]):
    arpabet_dict = {
        "AA": "ɑ",
        "AE": "æ",
        "AH": "ʌ",
        "AO": "ɔ",
        "AW": "aʊ",
        "AX": "ə",
        "AXR": "ɚ",
        "AY": "aɪ",
        "EH": "ɛ",
        "ER": "ɝ",
        "EY": "eɪ",
        "IH": "ɪ",
        "IX": "ɨ",
        "IY": "i",
        "OW": "oʊ",
        "OY": "ɔɪ",
        "UH": "ʊ",
        "UW": "u",
        "UX": "ʉ",
        "B": "b",
        "CH": "tʃ",
        "D": "d",
        "DH": "ð",
        "DX": "ɾ",
        "EL": "l̩",
        "EM": "m̩",
        "EN": "n̩",
        "F": "f",
        "G": "g",
        "H": "h",
        "HH": "h",
        "JH": "dʒ",
        "K": "k",
        "L": "l",
        "M": "m",
        "N": "n",
        "NG": "ŋ",
        "NX": "ɾ̃",
        "P": "p",
        "Q": "ʔ",
        "R": "r",
        "S": "s",
        "SH": "ʃ",
        "T": "t",
        "TH": "θ",
        "V": "v",
        "W": "w",
        "WH": "ʍ",
        "Y": "j",
        "Z": "z",
        "ZH": "ʒ",
    }

    try:
        phonemes = phonemes[: phonemes.index("#")]
    finally:
        return "".join([arpabet_dict[trim_numbers(p)] for p in phonemes])


if len(sys.argv) < 3:
    print("Please include the cmudict directory and the path of the SQLite database you wish to write to.")
else:
    cmudict_dir = sys.argv[1]
    sqlite_db = sys.argv[2]

    cd_dict_path = os.path.join(cmudict_dir, "cmudict.dict")

    with open(cd_dict_path) as cd_dict:
        conn = sqlite3.connect(sqlite_db)

        d = [(trim_parens(s[0]), fix(s[1:])) for s in [line.split() for line in cd_dict.readlines()]]
        df = pd.DataFrame.from_records(data=d, columns=["word", "phonemes"])
        df.to_sql("english", conn, if_exists="replace", index=False)
