install-cross:
	cargo install -f cross

build-release: build-release-unix build-release-win

build-release-unix:
	cargo build --release

build-release-win: install-cross
	cross build --release --target x86_64-pc-windows-gnu