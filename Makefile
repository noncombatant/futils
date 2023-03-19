INSTALL_ROOT = /usr/local
INSTALL_ROOT = $(HOME)

default:
	cargo build
	cargo test
	cargo clippy

doc:
	cargo doc

install:
	cargo build --release
	mkdir -p $(INSTALL_ROOT)/bin
	cp target/release/futils $(INSTALL_ROOT)/bin
	strip $(INSTALL_ROOT)/bin/futils
	ln -f $(INSTALL_ROOT)/bin/futils $(INSTALL_ROOT)/bin/apply
	ln -f $(INSTALL_ROOT)/bin/futils $(INSTALL_ROOT)/bin/fields
	ln -f $(INSTALL_ROOT)/bin/futils $(INSTALL_ROOT)/bin/files
	ln -f $(INSTALL_ROOT)/bin/futils $(INSTALL_ROOT)/bin/filter
	ln -f $(INSTALL_ROOT)/bin/futils $(INSTALL_ROOT)/bin/map
	ln -f $(INSTALL_ROOT)/bin/futils $(INSTALL_ROOT)/bin/records
	ln -f $(INSTALL_ROOT)/bin/futils $(INSTALL_ROOT)/bin/reduce
	ln -f $(INSTALL_ROOT)/bin/futils $(INSTALL_ROOT)/bin/status
