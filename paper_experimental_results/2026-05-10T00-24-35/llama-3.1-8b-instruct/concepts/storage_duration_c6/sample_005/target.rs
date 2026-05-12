fn distinct_instances(depth: i32, prev_addr: Option<usize>) -> bool {
    let local: i32; /* no initializer */

    let addr = local as *mut _ as usize;

    if let Some(prev_addr) = prev_addr {
        if prev_addr != 0 && addr == prev_addr {
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