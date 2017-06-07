#Paths
NAME=rusty
VERSION=0.1
BUILD=build
PKG=$(BUILD)/pkg-debian

prerequisites:
	sudo apt install pkg-config libsqlite3-dev libssl-dev

build: src/*
	RUSTY_VERSION=$(VERSION) cargo build 

build-release: src/*
	cargo clean
	RUSTY_VERSION=$(VERSION) cargo build --release 

build-release-armv7: src/*
	cargo clean
	export OPENSSL_DIR=/tmp/openssl-1.1.0f/; \
	export OPENSSL_LIB_DIR=/tmp/openssl-1.1.0f/; \
	RUSTY_VERSION=$(VERSION) cargo build --release --target armv7-unknown-linux-gnueabihf

package: build-release
	cp target/release/rusty $(PKG)/usr/bin/
	@# Make checksum
	rm -f $(PKG)/DEBIAN/md5sums 
	find $(PKG)/ -type f ! -regex '.*.hg.*' ! -regex '.*?debian-binary.*' ! -regex '.*?DEBIAN.*' -printf '$(PKG)/%P ' | xargs md5sum | sed 's/build\/pkg-debian\///' > $(PKG)/DEBIAN/md5sums
	dpkg -b $(PKG)/ $(BUILD)/$(NAME)-$(VERSION)-$(shell uname -m).deb

package-armv7: build-release-armv7
	cp target/armv7-unknown-linux-gnueabihf/release/rusty $(PKG)/usr/bin/
	@# Make checksum
	rm -f $(PKG)/DEBIAN/md5sums 
	find $(PKG)/ -type f ! -regex '.*.hg.*' ! -regex '.*?debian-binary.*' ! -regex '.*?DEBIAN.*' -printf '$(PKG)/%P ' | xargs md5sum | sed 's/build\/pkg-debian\///' > $(PKG)/DEBIAN/md5sums
	dpkg -b $(PKG)/ $(BUILD)/$(NAME)-$(VERSION)-armv7.deb
