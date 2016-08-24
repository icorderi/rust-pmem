#!/bin/sh
set -e

if [ ! -d "$HOME/.cargo" ]; then
    mkdir $HOME/.cargo
fi

cp ./travis/cargo/config $HOME/.cargo/config

cat $HOME/.cargo/config
