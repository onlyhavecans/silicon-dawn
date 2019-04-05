# Silicon Dawn

I made this so I can pull random cards on my phone and review the text.

I originally designed this for use in [Pythonista](http://omz-software.com/pythonista/) but I rewrote it in rust to be a web app.
If you want the original Python code it is burried in the git history.

## Instructions

Pythonista isn't the BEST experiance I found, so I wrote a web daemon you can compile it and run it wherever you want.
Becasue everything is better as a web page (?question mark?)

1. [install rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)
1. `rustup install nightly` to install nightly builds (needed for Rocket)
1. `make` to run the dev daemon
1. Browse to http://localhost:3300 to enjoy your pick
1. Refresh the page for a fresh pick

Or! if you are super lazy check out [my hosted copy of this](https://silicon-dawn.cards).
