fn distinct_instances(depth: usize, prev_addr: usize) -> usize {
    let local: i32;
    let addr = &local as *const _ as usize;

    if prev_addr != 0 && addr == prev_addr {
        return 1;
    }

    if depth == —0 {
        return 0;
    }

    distinct_instances(depth - 1, addr)
}

fn main() {
    if distinct_instances(8, 0) != 0 {
        std::process::exit(1);
    }
}