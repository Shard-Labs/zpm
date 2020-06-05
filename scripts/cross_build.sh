#!/bin/bash
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

release_dir=target/$TARGET/release

echo TARGET=$TARGET
echo TAG=$tag
echo RELEASE_DIR=$release_dir

cross build --target $TARGET --release

cp $release_dir/$BINARY_NAME $stage/
cd $stage

tar czf $src/zpm-$tag-$TARGET.tar.gz *

cd $src
rm -rf $stage