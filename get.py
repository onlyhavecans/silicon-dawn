#!/usr/bin/env python3
"""
I didn't want to include the files in a git repo so I made this retriever.
Like anything that grabs files from the internet it is annoying on many fronts.
If Egypt Ernash moves this file it breaks; if cloudflare hates you, it breaks.
"""

from pathlib import Path
from zipfile import ZipFile
import requests

CARDS_URL = "http://egypt.urnash.com/media/blogs.dir/1/files/2018/01/The-Tarot-of-the-Silicon-Dawn.zip"
ZIP_FILE = "The-Tarot-of-the-Silicon-Dawn.zip"


def write_zip_from_url(url, filename):
    zip_data = requests.get(url)
    with open(filename, 'wb') as zip_file:
        zip_file.write(zip_data.content)


def extract_cards(zipfile):
    with ZipFile(zipfile) as zip_file:
        for file in zip_file.filelist:
            if "_MACOSX" not in file.filename and "sand-home" not in file.filename:
                zip_file.extract(file)


def main():
    if not Path(ZIP_FILE).exists():
        write_zip_from_url(CARDS_URL, ZIP_FILE)

    if ZipFile(ZIP_FILE).testzip() is not None:
        print("The zipfile is incomplete or damaged. Delete it and re-run script")
    else:
        extract_cards(ZIP_FILE)


if __name__ == '__main__':
    main()
