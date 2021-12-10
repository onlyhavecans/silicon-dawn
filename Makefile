IMAGE := skwrl/silicon-dawn:latest
CARDS := cards
BIN := ./target/release/silicon-dawn
DAWNZIP := The-Tarot-of-the-Silicon-Dawn.zip

all: update test docker-run

update:
	cargo update

test:
	cargo test
	cargo clippy -- -D warnings
	cargo fmt -- --check

docker-build: $(CARDS)
	docker build -t $(IMAGE) .

docker-run: build
	docker run -p 8080:3200 --name Makefile-Dawn $(IMAGE)

push: build
	docker push $(IMAGE)

$(BIN):
	cargo build --release

local-run: $(CARDS) $(BIN)
	cargo build
	cargo run

setup: $(CARDS)

$(DAWNZIP):
	wget "http://egypt.urnash.com/media/blogs.dir/1/files/2018/01/The-Tarot-of-the-Silicon-Dawn.zip"

$(CARDS): $(DAWNZIP)
	unzip -oj $(DAWNZIP) -x "__MACOSX/*" "*/sand-home*" -d $(CARDS)

clean:
	@rm -rf ./target