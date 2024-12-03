#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

use core::{u32, usize};
#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0xffff_ffc0_2200_0000;
const IMAGE_HEADER_SIZE: usize = 16;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_start = PLASH_START as *const u8;
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
        let code = unsafe { core::slice::from_raw_parts(apps_start.add(header_size + offset), app_size as usize) };
        offset += app_size as usize;
        println!("content: {:?}: ", code);
    }
    println!("Load payload ok!");
}