#Paths
NAME=rusty
VERSION=0.1
BUILD=build
PKG=$(BUILD)/pkg-debian

build: src/*
	RUSTY_VERSION=$(VERSION) cargo build 

build-release: src/*
	RUSTY_VERSION=$(VERSION) cargo build --release

package: build-release
	git tag $(VERSION)
	cp target/release/rusty $(PKG)/usr/bin/
	@# Make checksum
	rm -f $(PKG)/DEBIAN/md5sums 
	find $(PKG)/ -type f ! -regex '.*.hg.*' ! -regex '.*?debian-binary.*' ! -regex '.*?DEBIAN.*' -printf '$(PKG)/%P ' | xargs md5sum | sed 's/build\/pkg-debian\///' > $(PKG)/DEBIAN/md5sums
	dpkg -b $(PKG)/ $(BUILD)/$(NAME)-$(VERSION).deb
