fn distinct_instances(depth: i32, _prev_addr: usize) -> i32 {
    if depth == 0 {
        0
    } else {
        distinct_instances(depth - 1, 0)
    }
}

fn main() {
    if distinct_instances(8, 0) != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}