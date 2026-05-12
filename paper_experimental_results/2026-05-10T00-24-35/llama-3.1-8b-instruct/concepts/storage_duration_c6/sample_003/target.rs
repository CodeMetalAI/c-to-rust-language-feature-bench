fn distinct_instances(depth: i32, prev_addr: usize) -> i32 {
    let local = (prev_addr as *const _);
    let addr: usize = local as usize;

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
        return 1;
    }
    return 0;
}