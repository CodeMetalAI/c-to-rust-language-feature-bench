fn distinct_instances(depth: i32, prev_addr: Option<usize>) -> bool {
    let local = 0;
    let addr = local as *const _ as usize;

    if let Some(prev_addr) = prev_addr {
        if addr == prev_addr {
            return true;
        }
    }

    if depth == 0 {
        return false;
    }

    distinct_instances(depth - 1, Some(addr))
}

fn main() {
    if distinct_instances(8, None) {
        std::process::exit(1);
    }
}