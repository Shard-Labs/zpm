#!/bin/bash
set -e

tmp_dir=$(mktemp -d -t ci-XXXXXXXXXX)
project=$(cat /dev/urandom | tr -cd 'a-f0-9' | head -c 16)

cd $tmp_dir

zpm create $project
cd $project

zpm compile
zpm compute
zpm setup
zpm export-verifier
zpm generate-proof
zpm verify
zpm clean

rm -rf $tmp_dir