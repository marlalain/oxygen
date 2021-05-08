C = rustc
IN = touch.rs
OUT = touch
# Following flags can be removed if the target system has the necessary libraries.
# TODO add options
FLAGS = -C target-feature=+crt-static

all:
	$(C) $(IN) -o $(OUT) $(FLAGS)
