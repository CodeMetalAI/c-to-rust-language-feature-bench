fn distinct_instances(depth: i32, prev_addr: *const ()) -> i32 {
    let local: i32; /* no initializer */

    let addr: *const _ = &local as *const _;

    if prev_addr != std::ptr::null() && addr == prev_addr {
        1
    } else if depth == 0 {
        0
    } else {
        distinct_instances(depth - 1, addr)
    }
}

fn main() {
    if distinct_instances(8, std::ptr::null()) != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}