fn distinct_instances(depth: u32, prev_addr: usize) -> bool {
    let local: i32; // no initializer

    let addr: usize = &local as *const i32 as usize;

    if prev_addr != 0 && addr == prev_addr {
        return true;
    }

    if depth == 0 {
        return false;
    }

    distinct_instances(depth - 1, addr)
}

fn main() {
    if distinct_instances(8, 0) {
        std::process::exit(1);
    }
}