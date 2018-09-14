pick:
	pipenv run python pick.py
DUMMY: pick

setup: The\ Tarot\ of\ the\ Silicon\ Dawn
	pipenv install
	pipenv run python get.py

web:
	cargo run --release
DUMMY: web
