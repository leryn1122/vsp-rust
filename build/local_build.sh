#!/usr/bin/env bash

cd -P "$(dirname ${0-$BASHSOURCE})/.." || exit 1

RUSTFLAGS='-C prefer-dynamic' \
  cargo build --release
mkdir -p release/{bin,conf,include,lib}
find target/release/ -maxdepth 1 -type f -name vsp* -perm -750 -exec cp -ar {} release/bin \;
awk -F ',' '{printf "cp -ar %s %s\n", $1, $2}' build/conf/list.csv

[ `command -v upx` ] && \
find release/bin/ -maxdepth 1 -type f -exec upx {} \;

mkdir -p /usr/lib/vsp /etc/alternatives

ln -sf $PWD/release                           /usr/lib/vsp/vsp-0.0.1
ln -sf /usr/lib/vsp/vsp-0.0.1                 /usr/lib/vsp/default-vsp
ln -sf /usr/lib/vsp/default-vsp/bin/vspc      /etc/alternatives/vspc
ln -sf /usr/lib/vsp/default-vsp/bin/vspr      /etc/alternatives/vspr
ln -sf /usr/lib/vsp/default-vsp/bin/vsps      /etc/alternatives/vsps
ln -sf /usr/lib/vsp/default-vsp/bin/vspstk    /etc/alternatives/vspstk
ln -sf /usr/lib/vsp/default-vsp/bin/vspx      /etc/alternatives/vspx
ln -sf /etc/alternatives/vspc                 /usr/bin/vspc
ln -sf /etc/alternatives/vspr                 /usr/bin/vspr
ln -sf /etc/alternatives/vsps                 /usr/bin/vsps
ln -sf /etc/alternatives/vspstk               /usr/bin/vspstk
ln -sf /etc/alternatives/vspx                 /usr/bin/vspx

echo "Compile finished." ;