#!/usr/bin/env bash
set -e

echo "Building Rust QEMU reachability plugin..."

cd qemu-plugin
cargo build --release
cd ..

cp qemu-plugin/target/release/libqemu_reachability_plugin.so ./libreachability_rust.so

echo "✅ Rust plugin built: libreachability_rust.so"
echo ""
echo "Usage:"
echo "  qemu-x86_64 -plugin ./libreachability_rust.so,output=reach.txt \\"
echo "    /usr/bin/rustc sample.rs"
echo ""
echo "With memory regions:"
echo "  qemu-x86_64 -plugin ./libreachability_rust.so,\\"
echo "    input_base=0x7fff0000,input_size=4096,\\"
echo "    output_base=0x7ffe0000,output_size=8192,\\"
echo "    output=reach.txt \\"
echo "    /usr/bin/rustc sample.rs"
