#! /usr/bin/env bash

set -e

# https://stackoverflow.com/a/17902999/6292472
files=$(shopt -s nullglob dotglob; echo deps_repo/*)
if (( ${#files} ))
then
    echo "deps_repo contains files"
else
    echo "deps_repo is empty (or does not exist or is a file)"
    exit 1
fi

for i in $(find deps_repo -type f -name "*.z64"); do
    [ -f "$i" ] || break
    echo "Processing:" $i

    ipl3checksum check  $i

    echo
done
