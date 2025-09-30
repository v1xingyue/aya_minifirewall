fn main() {
    aya_build::BpfBuilder::new()
        .expect("Failed to create BpfBuilder")
        .build()
        .expect("Failed to build eBPF program");
}
