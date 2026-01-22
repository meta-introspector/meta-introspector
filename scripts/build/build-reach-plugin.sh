#!/usr/bin/env bash
set -e

QEMU_DIR="/mnt/data1/nix/time/2024/06/28/qemu"

echo "Building QEMU reachability plugin..."

if [ ! -d "$QEMU_DIR" ]; then
    echo "Error: QEMU directory not found at $QEMU_DIR"
    exit 1
fi

gcc -shared -fPIC \
    -o libreachability.so \
    reachability_plugin.c \
    -I"$QEMU_DIR/include" \
    $(pkg-config --cflags --libs glib-2.0) \
    -O2

echo "✅ Plugin built: libreachability.so"
echo ""
echo "This plugin tracks byte-level data flow:"
echo "  • Which input bytes reach each output byte"
echo "  • Which instructions touched each output byte"
echo ""
echo "Usage:"
echo "  qemu-x86_64 -plugin ./libreachability.so,output=reach.txt \\"
echo "    /usr/bin/rustc sample.rs"
