all: target/release/alfred-pods
.PHONY: all

target/release/alfred-pods:
	cargo build --release
.PHONY: target/release/alfred-pods

install: target/release/alfred-pods
	./install-workflow.sh target/release/alfred-pods
.PHONY: install

update-plist:
	./install-workflow.sh --update-plist
.PHONY: update-plist

clean:
	cargo clean
.PHONY: clean
