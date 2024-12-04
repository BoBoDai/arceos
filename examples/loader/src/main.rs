#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]

use core::{u32, usize};
#[cfg(feature = "axstd")]
use axstd::println;
use axstd::process::exit;

const PLASH_START: usize = 0xffff_ffc0_2200_0000;
const RUN_START: usize = 0xffff_ffc0_8010_0000;
const IMAGE_HEADER_SIZE: usize = 16;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_start = PLASH_START as *const u8;
    let run_start = RUN_START;

    let header_size = IMAGE_HEADER_SIZE;
    let header = unsafe { core::slice::from_raw_parts(apps_start, header_size) };
    let apps_num = &header[0..2];
    let apps_num = u32::from_be_bytes([0, 0, apps_num[0], apps_num[1]]);
    println!("apps num: {}", apps_num);
    let mut offset = 0;
    for n in 0..apps_num {
        let start = (2 + 4 * n) as usize;
        let end = start + 4;
        let app_size = &header[start..end];
        let app_size = u32::from_be_bytes([app_size[0], app_size[1], app_size[2], app_size[3]]);
        println!("Load payload ...");
        let load_code = unsafe { core::slice::from_raw_parts(apps_start.add(header_size + offset), app_size as usize) };
        println!("load code {:?}; address [{:?}]", load_code, load_code.as_ptr());
        let run_code = unsafe {
            core::slice::from_raw_parts_mut((run_start + offset) as *mut u8, app_size as usize)
        };
        run_code.copy_from_slice(load_code);
        println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());
        offset += app_size as usize;
    }
    println!("Load payload ok!");
    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_shutdown as usize);

    println!("Execute app ...");
    let arg0: u8 = b'A';

    // execute app
    unsafe {
        core::arch::asm!("
        li      t0, {abi_num}
        slli    t0, t0, 3
        la      t1, {abi_table}
        add     t1, t1, t0
        ld      t1, (t1)
        jalr    t1
        li      t2, {run_start}
        jalr    t2
        j       .",
        run_start = const RUN_START,
        abi_table = sym ABI_TABLE,
        abi_num = const SYS_TERMINATE,
        in("a0") arg0,
        )
    }
}

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;
static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe { ABI_TABLE[num] = handle; }
}

fn abi_hello() {
    println!("[ABI:Hello] Hello. Apps!");
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
}

fn abi_shutdown() {
    println!("Bye~");
    exit(0);
}