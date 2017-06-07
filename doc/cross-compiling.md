# Cross-compiling

[Documentation](https://github.com/japaric/rust-cross)

Host: 
- ubuntu x86_64

Target:
- raspbian armv7

## Basic

    mkdir .cargo
    cat > .cargo/config << EOF
    [target.armv7-unknown-linux-gnueabihf]
    linker = "arm-linux-gnueabihf-gcc"
    EOF

    cargo build --target armv7-unknown-linux-gnueabihf

## Openssl

We must pass shared option when configuring openssl compilation (this will make -fPIC parameter be passed to the compiler).

    wget https://www.openssl.org/source/openssl-1.1.0f.tar.gz
    tar xzvf openssl-1.1.0f.tar.gz 
    cd openssl-1.1.0f/
    export MACHINE=armv7
    export ARCH=arm
    export CC=arm-linux-gnueabihf-gcc
    ./config shared
    make
    export OPENSSL_LIB_DIR=/tmp/openssl-1.0.1t/
    export OPENSSL_INCLUDE_DIR=/tmp/openssl-1.0.1t/include

And then use classical cross-compiling method
