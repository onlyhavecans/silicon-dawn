CARDS := The-Tarot-of-the-Silicon-Dawn
DAWNZIP := The-Tarot-of-the-Silicon-Dawn.zip

run: $(CARDS)
	cargo +nightly build
	cargo +nightly run
DUMMY: run

build: $(CARDS)
	cargo +nightly build --release

setup: $(CARDS)
DUMMY: setup

$(DAWNZIP):
	wget "http://egypt.urnash.com/media/blogs.dir/1/files/2018/01/The-Tarot-of-the-Silicon-Dawn.zip"

$(CARDS): $(DAWNZIP)
	unzip -oj $(DAWNZIP) -x "__MACOSX/*" "*/sand-home*" -d $(CARDS)

clean:
	@rm -rf $(DAWNDIR) $(CARDS)
DUMMY: clean
