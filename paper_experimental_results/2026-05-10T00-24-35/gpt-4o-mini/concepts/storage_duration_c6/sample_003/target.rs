fn distinct_instances(depth: i32, prev_addr: *const i32) -> i32 {
    let local: i32; // no initializer
    let addr = &local as *const i32;

    if prev_addr != std::ptr::null() && addr == prev_addr {
        return 1;
    }

    if depth == 0 {
        return 0;
    }

    distinct_instances(depth - 1, addr)
}

fn main() {
    if distinct_instances(8, std::ptr::null()) != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}