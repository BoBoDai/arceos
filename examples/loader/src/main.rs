#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

use core::{u32, usize};
#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0xffff_ffc0_2200_0000;
const IMAGE_HEADER_SIZE: usize = 8;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_start = PLASH_START as *const u8;
    let header_size = IMAGE_HEADER_SIZE;
    let header = unsafe { core::slice::from_raw_parts(apps_start, header_size) };
    let size = &header[0..4];
    let apps_size = u32::from_be_bytes([size[0], size[1], size[2], size[3]]);
    println!("Load payload ...");

    let code = unsafe { core::slice::from_raw_parts(apps_start.add(header_size), apps_size as usize) };
    println!("content: {:?}: ", code);

    println!("Load payload ok!");
}