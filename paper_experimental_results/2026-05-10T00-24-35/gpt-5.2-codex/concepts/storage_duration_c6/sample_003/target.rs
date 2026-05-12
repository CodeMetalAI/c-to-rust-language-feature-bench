use std::mem::MaybeUninit;

type Uptr = usize;

#[inline(never)]
fn distinct_instances(depth: i32, prev_addr: Uptr) -> i32 {
    let local = MaybeUninit::<i32>::uninit();
    let addr = &local as *const _ as Uptr;

    if prev_addr != 0 && addr == prev_addr {
        return 1;
    }

    if depth == 0 {
        return 0;
    }

    let res = distinct_instances(depth - 1, addr);
    if res != 0 {
        return res;
    }
    0
}

fn main() {
    if distinct_instances(8, 0) != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}