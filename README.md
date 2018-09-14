# Silicon Dawn

I made this so I can pull random cards on my phone and review the text.

I also want to make a card reference out of this as well.

I designed this for use in [Pythonista](http://omz-software.com/pythonista/) but it's basic enough to be portable

## Instructions

## Pythonista

1. Copy this whole folder into Pythonista
1. Open `get.py` and run it
1. Create a home screen icon or favorite for `pick.py`
1. Enjoy random cards

### Mac/Linux

1. `brew install python pipenv` or do whatever you do to python
1. Clone/download this somewhere
1. `make setup` in the directory to install deps & get cards
1. `make` to get a random card and description displayed

## Web Service

Pythonista isn't the BEST experiance I found, so I wrote a web daemon you can compile it and run it wherever you want.
Becasue everything is best as a web page (?question mark?)
I run this on a random server I bookmarked on my phone.


Why did I write it in rust?
Because _I hate myself_.
I have no better answer.
I wouldn't recommend doing what I did.
I originally was going to write a small flask app or even write it in go, which I normally do, and it's super easy to do...
Instead I wanted to experiment and wasted an entire day just to produce this beast.
It's some of the most painful code I've ever written.
Either way it's written, and my regrets are on display.
So I hope you enjoy it.
My Pain.

1. [install rust](https://doc.rust-lang.org/1.29.0/cargo/getting-started/installation.html)
1. `make setup` to get the cards
1. `make web` to run the daemon
1. Browse to http://localhost:3000 to enjoy your pick
1. Refresh the page for a fresh pick
