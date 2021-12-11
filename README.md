# Silicon Dawn

I made this so I can pull random cards on my phone and review the text.

I originally designed this for use in [Pythonista](http://omz-software.com/pythonista/), but I rewrote it in rust to be a web app.
If you want the original Python code it is buried in the git history.

## Instructions

Pythonista isn't the BEST experience I found, so I wrote a web daemon you can compile it and run it wherever you want.
Because everything is better as a web page (?question mark?)

### Docker

I publish the docker container at [DockerHub](https://hub.docker.com/r/skwrl/silicon-dawn).
It is fully self-contained and uses port 3200 internally.

You can spin up a copy however you choose to do a docker or to build your own

1. Ensure you have docker for your playform installed
1. `make docker-run` to build and deploy locally
1. browse to [http://localhost:8080]

### Local Binary

1. [install rust](https://rustup.rs)
1. `make local-run` to download cards, compile, and go!
1. Browse to [http://localhost:3200] to enjoy your pick
1. Refresh the page for a fresh pick

Or! if you are super lazy check out [my hosted copy of this](https://silicon-dawn.cards).
