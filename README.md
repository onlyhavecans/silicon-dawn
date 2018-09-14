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

## Web Part

Pythonista isn't the BEST experiance I found, so I wrote a web daemon you can compile it and run it wherever you want.
Becasue everything is best as a web page (?question mark?)
I run this on a random server I bookmarked on my phone.


Why did I write it in rust?
Because _I hate myself_.
I have no better answer.
I wouldn't recommend doing what I did.
Don't use my code as an example and don't write web daemons in rust.
I originally was going to write a small flask app or even write it in go, which I normally do, and it's super easy to do...
Instead I wanted to experiment and wasted half a day just to produce this beast.
It's some of the most painful code I've ever written.
See where in `list_all_jpgs()` where I `cards.push(format!("{}", entry.file_name().to_str().unwrap()));`?
I regret everything about that line but after spending over an hour fighting the borrow checker I gave up on life and did that.
The function `render_card_picks()` is me just wanting the suffering to end and it shows.
Seriously... fuck this code and fuck rust in the face.
I wanted to refactor `return_card()` to be less monolithic and be more testable but I can't practice that level of self harm.
I'd love to write tests for all the parts of this so I can refacter lots of it but again...
I don't hate myself _that much_.
Speaking of pain I started writing this directly in hyper.rs.
Have you ever written boilerplate for your boilerplate?
I'm sure it's amazing to have a flexible and extensible web framework that can do everything.
But does it require writing an LoC comparible to Apache Tomcat's just to make something this basic?
Now I'm just venting angry noises becasue I'm hangry.
I started this after lunch and now it's super late.
The only advantage is that somehow I can't seem to get a directory traversal exploit on it.
I'm sure it's possible but for some reason I could get down a directory.
So... plus one big fat question mark?
Either way it's written, and my regrets are on display.
So I hope you enjoy it.
My Pain.

1. [install rust](https://doc.rust-lang.org/1.29.0/cargo/getting-started/installation.html)
1. `make web` to run the daemon
1. browse to http://localhost:3000 to enjoy your pick
1. Refresh the page for a fresh pick
