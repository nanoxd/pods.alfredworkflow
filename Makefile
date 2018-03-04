all: build
.PHONY: all

build:
	cargo build --release
.PHONY: build

install: build
	./install-workflow.sh target/release/alfred-pods
.PHONY: install

update-plist:
	./install-workflow.sh --update-plist
.PHONY: update-plist

clean:
	cargo clean
	rm -f pods.alfredworkflow
.PHONY: clean

release: clean build
	cp target/release/alfred-pods .
	zip -9 -r pods.alfredworkflow icon.png info.plist alfred-pods
	rm alfred-pods
