default:
	cargo build && cargo clippy

install:
	cargo build --release
	cp target/release/futils ~/bin
	strip ~/bin/futils
	ln -f ~/bin/futils ~/bin/apply
	ln -f ~/bin/futils ~/bin/files
	ln -f ~/bin/futils ~/bin/filter
	ln -f ~/bin/futils ~/bin/records
