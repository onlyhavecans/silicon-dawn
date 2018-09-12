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


def main():
    if not Path(ZIP_FILE).exists() and ZipFile(ZIP_FILE).testzip() is None:
        zip_data = requests.get(CARDS_URL)
        with open(ZIP_FILE, 'wb') as zip_file:
            zip_file.write(zip_data.content)

    with ZipFile(ZIP_FILE) as zip_file:
        for file in zip_file.filelist:
            if "_MACOSX" not in file.filename and "sand-home" not in file.filename:
                zip_file.extract(file)


if __name__ == '__main__':
    main()
