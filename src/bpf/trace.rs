// this will exe in sandboxed within kernal
#![no_std]
#![no_main]

use aya_bpf::{bindings::*, macros::tracepoint, programs::TracePointContext};
use aya_log_ebpf::info;

#[tracepoint(name = "hello_bpf")]
pub fn hello_bpf(ctx: TracePointContext) -> u32 {
    match unsafe { try_hello_bpf(ctx) } {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

unsafe fn try_hello_bpf(_ctx: TracePointContext) -> Result<(), u32> {
    // This will print "Hello, World" to the trace pipe.
    info!(_ctx, "Hello, World");
    Ok(())
}

// License required for the program to be loaded into the kernel.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::arch::asm!("ud2");
    loop {}
}
