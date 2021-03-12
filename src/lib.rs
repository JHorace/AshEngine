#[macro_use]
extern crate memoffset;

pub mod geometry;
pub mod render_sequence;
pub mod vulkan;

#[no_mangle]
pub extern "C" fn super_dark_sum(a: u64, b: u64) -> u64 {
    a + b
}
