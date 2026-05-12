fn distinct_instances(depth: i32, prev_addr: usize) -> i32 {
    let local: i32;
    let addr: usize = local as usize;

    if prev_addr!= 0 && addr == prev_addr {
        1
    } else if depth == 0 {
        0
    } else {
        distinct_instances(depth - 1, addr)
    }
}

fn main() {
    if distinct_instances(8, 0)!= 0 {
        std::process::exit(1);
    }
}