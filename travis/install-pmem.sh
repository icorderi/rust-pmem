#!/bin/sh
set -e

# check to see if protobuf folder is empty
if [ ! -d "$HOME/nvml/lib" ]; then
    version=1.1
    wget https://github.com/pmem/nvml/archive/$version.tar.gz -O nvml-$version.tar.gz
    tar -xzvf nvml-$version.tar.gz
    cd nvml-$version
    make
    make install prefix=$HOME/nvml
else
    echo 'Using cached nvml.';
fi

ls -R $HOME/nvml/lib
