use std::mem::MaybeUninit;
use std::process;

type Uptr = usize;

fn distinct_instances(depth: i32, prev_addr: Uptr) -> i32 {
    let local = MaybeUninit::<i32>::uninit();
    let addr = &local as *const _ as Uptr;

    if prev_addr != 0 && addr == prev_addr {
        return 1;
    }

    if depth == 0 {
        return 0;
    }

    distinct_instances(depth - 1, addr)
}

fn main() {
    if distinct_instances(8, 0) != 0 {
        process::exit(1);
    }
}