fn distinct_instances(depth: usize, prev_addr: Option<*const i32>) -> bool {
    let local = 0;

    let addr = local as *const i32 as usize;

    if let Some(prev_addr) = prev_addr {
        if prev_addr == &local as *const i32 as usize {
            return true;
        }
    }

    if depth == 0 {
        return false;
    }

    distinct_instances(depth - 1, Some(&local as *const i32))
}

fn main() {
    if distinct_instances(8, None) {
        std::process::exit(1);
    }
    std::process::exit(0);
}