#!/bin/zsh

LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/lib" Z3_SYS_Z3_HEADER="/opt/homebrew/include/z3.h" cargo test
