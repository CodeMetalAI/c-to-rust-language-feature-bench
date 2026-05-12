use std::mem::MaybeUninit;

fn distinct_instances(depth: i32, prev_addr: usize) -> i32 {
    let local: MaybeUninit<i32> = MaybeUninit::uninit();
    let addr = (&local as *const MaybeUninit<i32>) as usize;

    if prev_addr != 0 && addr == prev_addr {
        return 1;
    }

    if depth == 0 {
        return 0;
    }

    distinct_instances(depth - 1, addr)
}

fn main() {
    let result = distinct_instances(8, 0);
    if result != 0 {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}