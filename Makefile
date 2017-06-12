#Paths
NAME=rusty
VERSION=$(shell git describe --tags)
BUILD=/tmp
PKG=$(BUILD)/pkg-debian

define CONTROL_FILE
Package: rusty
Version: $(VERSION)
Maintainer: Yann Fery yann@fery.me
Priority: optional
Architecture: all
Depends: libssl1.0.0(>= 1.0), libsqlite3-0(>=3.8)
Description: Managing podcast feeds
endef
export CONTROL_FILE

build: src/*
	RUSTY_VERSION=$(VERSION) cargo build 

build-release: src/*
	export OPENSSL_INCLUDE_DIR=/usr/include/openssl/; \
	export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu/; \
	export CC=/usr/bin/cc; \
	RUSTY_VERSION=$(VERSION) cargo build --release 

build-release-armv7: src/*
	export OPENSSL_DIR=/tmp/openssl-1.0.2l/; \
	export OPENSSL_LIB_DIR=/tmp/openssl-1.0.2l/; \
	export CPPFLAGS="-I/tmp/openssl-1.0.2l/include/"; \
	export LDFLAGS="-L/tmp/openssl-1.0.2l/"; \
	export LIBS="-lssl -lcrypto"; \
	export CC=arm-linux-gnueabihf-gcc; \
	RUSTY_VERSION=$(VERSION) cargo build --release --target armv7-unknown-linux-gnueabihf

package-control:
	rm -r $(PKG)
	mkdir -p $(PKG)/DEBIAN
	mkdir -p $(PKG)/usr/bin/
	echo "$$CONTROL_FILE" > $(PKG)/DEBIAN/control

package: package-control build-release
	cp target/release/rusty $(PKG)/usr/bin/
	@# Make checksum
	rm -f $(PKG)/DEBIAN/md5sums 
	find $(PKG)/ -type f ! -regex '.*.hg.*' ! -regex '.*?debian-binary.*' ! -regex '.*?DEBIAN.*' -printf '$(PKG)/%P ' | xargs md5sum | sed 's/build\/pkg-debian\///' > $(PKG)/DEBIAN/md5sums
	dpkg -b $(PKG)/ build/$(NAME)-$(shell uname -m)-$(VERSION).deb

package-armv7: package-control build-release-armv7
	cp target/armv7-unknown-linux-gnueabihf/release/rusty $(PKG)/usr/bin/
	@# Make checksum
	rm -f $(PKG)/DEBIAN/md5sums 
	find $(PKG)/ -type f ! -regex '.*.hg.*' ! -regex '.*?debian-binary.*' ! -regex '.*?DEBIAN.*' -printf '$(PKG)/%P ' | xargs md5sum | sed 's/build\/pkg-debian\///' > $(PKG)/DEBIAN/md5sums
	dpkg -b $(PKG)/ build/$(NAME)-armv7-$(VERSION).deb

packages: package-armv7 package
