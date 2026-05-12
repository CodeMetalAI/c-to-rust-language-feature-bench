use std::process::exit;

fn distinct_instances(depth: i32, prev_addr: usize) -> i32 {
    let local: i32 = 0;
    let addr = std::ptr::addr_of!(local) as usize;

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
        exit(1);
    }
    exit(0);
}