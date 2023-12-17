#! /usr/bin/env bash

set -e

for i in tests/dummytests/*/*; do
    [ -f "$i" ] || break
    echo "Processing:" $i

    # Get CIC kind based on folder's name
    CIC_KIND=$(basename $(dirname $i))
    echo "CIC kind" $CIC_KIND

    ./bindings/c/tests/test_checksum.elf $i $CIC_KIND
    echo
done
