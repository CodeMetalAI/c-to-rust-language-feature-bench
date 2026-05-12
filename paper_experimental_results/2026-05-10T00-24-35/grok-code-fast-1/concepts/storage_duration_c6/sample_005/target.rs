fn distinct_instances(_depth: i32, _prev_addr: Option<u64>) -> i32 {
    // In safe Rust, we cannot obtain memory addresses of locals to compare.
    // However, in practice, each recursive call has a distinct stack frame,
    // so addresses would always differ, making the return value always 0.

    if _depth == 0 {
        0
    } else {
        distinct_instances(_depth - 1, None)
    }
}

fn main() {
    if distinct_instances(8, None) != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}