use std::rc::Rc;
use std::cell::RefCell;

type Uptr = usize;

fn distinct_instances(depth: i32, prev_addr: Uptr) -> i32 {
    let local: i32;
    
    // Get the address of the local variable
    let addr = &local as *const i32 as Uptr;
    
    if prev_addr != 0 && addr == prev_addr {
        return 1;
    }
    
    if depth == 0 {
        return 0;
    }
    
    // Use recursion to create new stack frames
    distinct_instances(depth - 1, addr)
}

fn main() {
    if distinct_instances(8, 0) != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}