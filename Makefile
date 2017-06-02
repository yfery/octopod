.PHONY: .build

#Paths
NAME=rusty
BUILD=build
PKG=$(BUILD)/pkg-debian

all: src/*
	cargo build --release
	cp target/release/rusty $(PKG)/usr/bin/
	@# Make checksum
	rm -f $(PKG)/DEBIAN/md5sums 
	find $(PKG)/ -type f ! -regex '.*.hg.*' ! -regex '.*?debian-binary.*' ! -regex '.*?DEBIAN.*' -printf '$(PKG)/%P ' | xargs md5sum | sed 's/build\/pkg-debian\///' > $(PKG)/DEBIAN/md5sums
	dpkg -b $(PKG)/ $(BUILD)/$(NAME).deb
	
