// User Space Loader -> main program loader to load and run the eBPF program:
use aya::{include_bytes_aligned, Bpf};
use aya::programs::TracePoint;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the compiled eBPF bytecode.
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../target/bpfel-unknown-none/debug/trace"
    ))?;
    
    // Attach the eBPF program to the sys_enter_execve tracepoint.
    let program: &mut TracePoint = bpf.program_mut("hello_bpf").unwrap().try_into()?;
    program.load()?;
    program.attach("syscalls", "sys_enter_execve")?;

    println!("eBPF program loaded. Check output in trace pipe.");
    
    // Wait for a signal (like Ctrl+C) to exit.
    signal::ctrl_c().await?;
    Ok(())
}
