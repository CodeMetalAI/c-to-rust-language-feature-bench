fn distinct_instances(depth: i32, prev_addr: usize) -> i32 {
    let local = 0;  // Local variable, we need to initialize to use its address in Rust
    let addr = &local as *const i32 as usize;

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
        std::process::exit(1);
    }
}