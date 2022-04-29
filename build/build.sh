#!/usr/bin/env bash

cd -P "$(dirname ${0-$BASHSOURCE})/.." || exit 1

echo "#########################################################################"
echo "##  Start to build release pack...                                     ##"
echo "#########################################################################"
echo ""
echo "Release path: $PWD"

find . -maxdepth 1 -type d -name "release" -exec rm -rf {} \;
mkdir -p release/{bin,conf,include,lib}

cp -ar target/release/vsp*  release/bin
cp -ar target/release/lib*  release/lib
cp -ar conf                 release/
cp -ar LICENSE              release/