## eBPF
eBPF (extended Berkeley Packet Filter) is a technology in the Linux kernel that allows you to run sandboxed programs (eBPF programs) safely within the kernel without changing kernel source code or adding additional modules.

## How eBPF Works:
- eBPF Programs: These are small, bytecode programs written in C or other languages like Rust and then compiled into eBPF bytecode. They can be attached to various hooks in the kernel, such as network events, system calls, and function entry/exit points.

- Verification and Safety: Before an eBPF program is loaded into the kernel, a verifier checks it to ensure it is safe to run, does not contain loops, and adheres to resource constraints.

- Maps and Helpers: eBPF uses data structures called "maps" to store and share data between the kernel and user space, and "helper functions" to interact with the kernel safely.


## How to run and load eBPF to kernal

### Install LLVM and Clang: These tools are needed to compile Rust code to eBPF bytecode:

``` 
sudo apt-get install llvm clang libclang-dev linux-headers-$(uname -r) 
```

### Compile the eBPF Program

- You will need cargo xtask to compile the eBPF program properly. To install it, run:
``` 
cargo install cargo-xtask 
```

- compile the project
``` 
cargo xtask bpf
cargo build
```

trace.rs is not successfully converted to bytecode

- Now run User-Space Loader to load this bytecode to kernal
```
sudo ./target/debug/trace 
```

### Check output
``` 
sudo cat /sys/kernel/debug/tracing/trace_pipe
```




