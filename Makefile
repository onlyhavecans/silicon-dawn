CARDS := The-Tarot-of-the-Silicon-Dawn
DAWNZIP := The-Tarot-of-the-Silicon-Dawn.zip

web: $(CARDS)
	cargo run --release
DUMMY: web

setup: $(CARDS)
DUMMY: setup

$(DAWNZIP):
	wget "http://egypt.urnash.com/media/blogs.dir/1/files/2018/01/The-Tarot-of-the-Silicon-Dawn.zip"

$(CARDS): $(DAWNZIP)
	unzip "The-Tarot-of-the-Silicon-Dawn.zip" -x "__MACOSX/*" "*/sand-home*"
	mv -f "The Tarot of the Silicon Dawn" $(CARDS)

clean:
	@rm -rf $(DAWNDIR) $(DAWNZIP)
DUMMY: clean
