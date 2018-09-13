#!/usr/bin/env python3
"""
Pick a card, any card
"""
from PIL import Image
from pathlib import Path
import random

TAROT_FOLDER = "The Tarot of the Silicon Dawn"
CARD_GLOB = "*.jpg"
TEXT_ENDING = "-text.png"


def main():
    tarot_path = Path(TAROT_FOLDER)

    cards = [x for x in tarot_path.glob(CARD_GLOB) if "sand-home" not in x.stem]

    pick_front = random.choice(cards)
    pick_text = tarot_path.joinpath(f'{pick_front.stem}{TEXT_ENDING}')

    print(f'My pick is {pick_front.stem}')

    card_img = Image.open(str(pick_front))
    card_img.show()

    text_img = Image.open(str(pick_text))
    text_img.show()


if __name__ == '__main__':
    main()
