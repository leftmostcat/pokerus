#!/usr/bin/python

import os
import re
import sys


def main():
    cwd = os.path.dirname(os.path.realpath(__file__))
    home_dir = os.path.join(cwd, "..", "home")

    real_fn_regex = re.compile("HOME \d+ (\w+)\.txt")
    virt_fn_regex = re.compile("Text File : (\w+)")
    files_by_lang = {}

    lang_to_lang = {
        "JPN": "ja",
        "English": "en",
        "French": "fr",
        "Italian": "it",
        "German": "de",
        "Spanish": "es",
        "Korean": "ko",
        "Simp_Chinese": "zh-Hans",
        "Trad_Chinese": "zh-Hant",
    }

    for filename in os.listdir(home_dir):
        matches = real_fn_regex.match(filename)
        lang = matches.group(1)
        if lang is None or lang == "JPN_KANJI":
            continue

        lang = lang_to_lang[lang]
        if lang is None:
            continue

        filename = os.path.join(home_dir, filename)
        with open(filename, "r", encoding="utf-8") as input:
            files = {}

            reading_name = False
            current_file = None
            for raw_line in input:
                line = raw_line.rstrip()
                line = line.lstrip("\ufeff")
                if reading_name:
                    if line == "~~~~~~~~~~~~~~~":
                        reading_name = False

                        continue

                    matches = virt_fn_regex.match(line)
                    if matches is None:
                        sys.exit(1)

                    text_file = matches.group(1)
                    if text_file is None:
                        sys.exit(1)

                    current_file = text_file
                    files[current_file] = []

                    continue

                if line == "~~~~~~~~~~~~~~~":
                    current_file = None
                    reading_name = True

                    continue

                if current_file is not None:
                    files[current_file].append(raw_line)

            for file in files.keys():
                files[file].append("")

            files_by_lang[lang] = files

    out_dir = os.path.join(cwd, "..", "generated")
    for lang in files_by_lang.keys():
        for file in files_by_lang[lang].keys():
            filename = "{base}.txt".format(base=file, lang=lang)
            filename = os.path.join(out_dir, lang, filename)
            with open(filename, "w", encoding="utf-8") as out:
                out.writelines(files_by_lang[lang][file])


if __name__ == "__main__":
    main()
