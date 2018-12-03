#!/bin/bash

for i in d_*/ ; do
    echo "$i"
    cd $i
    cargo run
    cd ..
done