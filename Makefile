CARDS := The-Tarot-of-the-Silicon-Dawn
DAWNZIP := The-Tarot-of-the-Silicon-Dawn.zip

run: $(CARDS)
	cargo +nightly build
	cargo +nightly run
DUMMY: run

prod: build
	cargo +nightly run --release
DUMMY: prod

build: $(CARDS)
	cargo +nightly build --release

setup: $(CARDS)
DUMMY: setup

$(DAWNZIP):
	wget "http://egypt.urnash.com/media/blogs.dir/1/files/2018/01/The-Tarot-of-the-Silicon-Dawn.zip"

$(CARDS): $(DAWNZIP)
	unzip "The-Tarot-of-the-Silicon-Dawn.zip" -x "__MACOSX/*" "*/sand-home*"
	mv -f "The Tarot of the Silicon Dawn" $(CARDS)

clean:
	@rm -rf $(DAWNDIR) $(CARDS)
DUMMY: clean
