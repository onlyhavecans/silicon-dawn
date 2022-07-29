IMAGE := skwrl/silicon-dawn:latest
CARDS := cards
BIN := ./target/release/silicon-dawn
DAWNZIP := The-Tarot-of-the-Silicon-Dawn.zip

all: test docker-run

update:
	cargo update

test:
	cargo fmt -- --check
	cargo clippy -- -D warnings
	cargo test

docker-build: $(CARDS)
	docker build -t $(IMAGE) .

docker-run: docker-build
	docker run --rm -p 8080:3200 --name Makefile-Dawn $(IMAGE)

push:
	docker push $(IMAGE)

$(BIN):
	cargo build --release

local-run: $(CARDS) $(BIN)
	cargo run

setup: $(CARDS)

$(DAWNZIP):
	wget "http://egypt.urnash.com/media/blogs.dir/1/files/2018/01/The-Tarot-of-the-Silicon-Dawn.zip"

$(CARDS): $(DAWNZIP)
	unzip -oj $(DAWNZIP) -x "__MACOSX/*" "*/sand-home*" -d $(CARDS)

clean:
	cargo clean
