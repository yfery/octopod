# Building

Rusty depends on OpenSSL (or LibreSSL) and Sqlite3. The libraries and headers need to be present in the build environment for building. `pkg-config` is needed too as a regular development utility.

On debian/ubuntu/raspbian

    sudo apt install pkg-config libssl-dev libsqlite3-0 
    make build-release

Makefile is prefered as it initialises some environment variable before doing a `cargo build`

## Makefile options 

- `make build`: build a debug version
- `make build-release-armv7`: build an armv7 version on other host with cross-compiling method
- `make package`: build and create a debian package
- `make package-armv7`: build and create a debian package for armv7 with cross-compiling

## Misc

For sqlite3 packages needed by rusqlite([explanation](https://github.com/jgallagher/rusqlite#notes-on-building-rusqlite-and-libsqlite3-sys)):

## Cross-compiling

[Documentation](https://github.com/japaric/rust-cross)

Host: 
- ubuntu x86_64

Target:
- raspbian armv7

We must pass shared option when configuring openssl compilation (this will make -fPIC parameter be passed to the compiler).

Download 1.0.1x version as there is no 1.1 version packages in repository right now, and version 1.0.2 are not fully compatible.

    wget https://www.openssl.org/source/openssl-1.0.1u.tar.gz
    tar xzvf openssl-1.0.1u.tar.gz
    cd openssl-1.0.1u
    export MACHINE=armv7
    export ARCH=arm
    export CC=arm-linux-gnueabihf-gcc
    ./config shared
    make
    export OPENSSL_LIB_DIR=/tmp/openssl-1.0.1u/
    export OPENSSL_INCLUDE_DIR=/tmp/openssl-1.0.1u/include

For ssl support during curl compilation ([source](http://www.matteomattei.com/how-to-cross-compile-curl-library-with-ssl-and-zlib-support/))

    export CPPFLAGS="-I/tmp/openssl-1.0.1u/include/"
	export LDFLAGS="-L/tmp/openssl-1.0.1u/"; \
	export LIBS="-lssl -lcrypto"; \

We create a cargo configuration file 

    mkdir ~/.cargo
    cat > ~/.cargo/config << EOF
    [target.armv7-unknown-linux-gnueabihf]
    linker = "arm-linux-gnueabihf-gcc"
    EOF

And finally we can build 

    cargo build --target armv7-unknown-linux-gnueabihf

## Debugging

[Explanation](https://thekerneldiaries.com/2016/06/16/making-sure-the-correct-libraries-are-loaded-at-run-time/)

Print shared libraries dependencies 

    ldd <binary> 

Display dynamic part of ELF file

    readelf -d <binary>

To know options passed to package compilation

    more target/release/build/<package>/output
