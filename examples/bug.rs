#![no_std]
#![feature(start, lang_items, rustc_private, libc, default_alloc_error_handler)]

extern crate azul;
extern crate alloc;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! { loop {} }

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[global_allocator]
static ALLOC: libc_alloc::LibcAlloc = libc_alloc::LibcAlloc;

#[start]
fn main(_: isize, _: *const *const u8) -> isize {
    return 0;
}
