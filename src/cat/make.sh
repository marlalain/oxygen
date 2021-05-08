#!/bin/sh

CC=rustc

NAME=${PWD##*/} # sh magic;; TODO test on other shells
IN=$NAME".rs"
OUT=$NAME
DESTDIR="../../build/" # default value

# Following flags can be removed if the target system has the necessary libraries.
# TODO add options
FLAGS="-C target-feature=+crt-static"

#$(C) $(IN) -o $(DESTDIR)$(OUT) $(FLAGS)
echo "Building $NAME..."
$CC $IN -o $DESTDIR$OUT $FLAGS
