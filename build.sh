#!/bin/bash

set -e

echo "Building Aya Mini Firewall..."

# Build the eBPF program
echo "Building eBPF program..."
cd aya-minifirewall-ebpf
cargo build --release --target bpfel-unknown-none
cd ..

# Build the userspace program
echo "Building userspace program..."
cargo build --release

echo "Build complete!"
echo ""
echo "To run the firewall:"
echo "  sudo ./target/release/aya-minifirewall load --interface <interface>"
echo ""
echo "To manage rules:"
echo "  sudo ./target/release/aya-minifirewall block-ip <ip>"
echo "  sudo ./target/release/aya-minifirewall unblock-ip <ip>"
echo "  sudo ./target/release/aya-minifirewall block-port <port>"
echo "  sudo ./target/release/aya-minifirewall unblock-port <port>"
echo "  sudo ./target/release/aya-minifirewall list"
