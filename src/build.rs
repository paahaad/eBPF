// compile the eBPF program
use std::process::Command;

fn main() {
    // Compile the eBPF program using cargo bpf from the aya build tool.
    let status = Command::new("cargo")
        .args(&[
            "xtask",
            "bpf",
            "--target",
            "bpfel-unknown-none",
            "--features",
            "full",
        ])
        .status()
        .expect("failed to run cargo xtask bpf");

    if !status.success() {
        panic!("failed to compile eBPF program");
    }
}
