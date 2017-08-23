#Paths
NAME=octopod
VERSION=$(shell git describe --tags)
BUILD=/tmp
PKG=$(BUILD)/pkg-debian
OPENSSL=/tmp/openssl-1.0.1u/

define CONTROL_FILE
Package: octopod
Version: $(VERSION)
Maintainer: Yann Fery yann@fery.me
Priority: optional
Architecture: all
Depends: libssl1.0.0(>= 1.0), libsqlite3-0(>=3.8)
Description: Managing podcast feeds
endef
export CONTROL_FILE

build: src/*
	diesel migration run
	OCTOPOD_VERSION=$(VERSION) cargo build 

build-release: src/*
	export OPENSSL_INCLUDE_DIR=/usr/include/openssl/; \
	export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu/; \
	export CC=/usr/bin/cc; \
	diesel migration run
	OCTOPOD_VERSION=$(VERSION) cargo build --release 

package-control:
	rm -fr $(PKG)
	mkdir -p $(PKG)/DEBIAN
	mkdir -p $(PKG)/usr/bin/
	echo "$$CONTROL_FILE" > $(PKG)/DEBIAN/control
	mkdir -p $(PKG)/etc/bash_completion.d/
	cp assets/completion.bash $(PKG)/etc/bash_completion.d/$(NAME)

package: package-control build-release
	cp target/release/$(NAME) $(PKG)/usr/bin/
	@# Make checksum
	rm -f $(PKG)/DEBIAN/md5sums 
	find $(PKG)/ -type f ! -regex '.*.hg.*' ! -regex '.*?debian-binary.*' ! -regex '.*?DEBIAN.*' -printf '$(PKG)/%P ' | xargs md5sum | sed 's/build\/pkg-debian\///' > $(PKG)/DEBIAN/md5sums
	dpkg -b $(PKG)/ build/$(NAME)-$(shell uname -m)-$(VERSION).deb

test: src/*
	OCTOPOD_VERSION=$(VERSION) cargo test 
