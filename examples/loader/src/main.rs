#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]

use axstd::os::arceos::modules::axhal::mem::MemoryAddr;
#[cfg(feature = "axstd")]
use axstd::println;
use axstd::process::exit;
use core::mem;

const PLASH_START: usize = 0xffff_ffc0_2200_0000;

#[repr(C)]
#[derive(Debug, Clone)]
struct ImageHeader {
    magic_number: u32,
    app_size: u32,
    entry_offset: u64,
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let header_ptr = PLASH_START as *const ImageHeader;
    let header = unsafe { &*header_ptr };
    let app_size = header.app_size as usize;
    let entry_offset = header.entry_offset as usize;
    let header_size = mem::size_of::<ImageHeader>();

    let apps_start = PLASH_START.add(header_size + entry_offset) as *const u8;
    println!("Load payload ...");

    let load_code = unsafe { core::slice::from_raw_parts(apps_start, app_size) };
    // println!(
    //     "load code {:?}; address [{:?}]",
    //     load_code,
    //     load_code.as_ptr()
    // );

    // app running aspace
    // SBI(0x80000000) -> App <- Kernel(0x80200000)
    // va_pa_offset: 0xffff_ffc0_0000_0000
    const RUN_START: usize = 0xffff_ffc0_8010_0000;

    let run_code = unsafe { core::slice::from_raw_parts_mut(RUN_START as *mut u8, app_size) };
    run_code.copy_from_slice(load_code);
    // println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());

    println!("Load payload ok!");

    register_abi(SYS_HELLO, api_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_shutdown as usize);
    register_abi(SYS_PUTS, abi_puts as usize);
    println!("Execute app ...");

    // execute app
    unsafe {
        core::arch::asm!("
        la      a7, {abi_table}
        li      t2, {run_start}
        jalr    t2
        j       .",
        run_start = const RUN_START,
        abi_table = sym ABI_TABLE,
        )
    }
}

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;
const SYS_PUTS: usize = 4;

static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe {
        ABI_TABLE[num] = handle;
    }
}

fn api_hello() {
    println!("[ABI:Hello] Hello, Apps!")
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}")
}

fn abi_puts(s: &str) {
    println!("[ABI:Print] {s}")
}

fn abi_shutdown() {
    println!("Bye~");
    exit(0);
}
