#!/usr/bin/env bash
exec x86_64-w64-mingw32-gcc "$@" -Wl,--defsym,__ImageBase=0
