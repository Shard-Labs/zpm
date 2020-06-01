#!/bin/bash

#  x86_64-apple-darwin
#  x86_64-pc-windows-gnu
#  x86_64-unknown-linux-gnu

set -e

stage=$(mktemp -d -t cb-XXXXXXXXXX)
src=$(pwd)
tag=$(cat Cargo.toml | grep '^version' | awk '{print $3}' | sed -e 's/"//g')

case $TARGET in
    x86_64-pc-windows-gnu)
        BINARY_NAME=zpm.exe
        ;;
    *)
        BINARY_NAME=zpm
        ;;
esac

echo TARGET=$TARGET
echo TAG=$tag

rustup target install $TARGET
cargo build --target $TARGET --release

cp target/$TARGET/release/$BINARY_NAME $stage/
cd $stage

mkdir -p $src/artifacts
tar czf $src/artifacts/zpm-$tag-$TARGET.tar.gz *

cd $src
rm -rf $stage