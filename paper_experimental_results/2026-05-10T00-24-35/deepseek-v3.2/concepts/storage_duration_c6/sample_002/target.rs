use std::collections::HashSet;

fn distinct_instances(depth: u32, prev_addr: usize, seen_addresses: &mut HashSet<usize>) -> i32 {
    let local: usize = 42; // Dummy value to get a stack address
    let addr = &local as usize;
    
    if !prev_addr == 0 && addr == prev_addr {
        return 1;
    }
    
    if depth == 0 {
        return 0;
    }
    
    seen_addresses.insert(addr);
    
    return distinct_instances(depth - 1, addr, seen_addresses);
}

fn main() {
    let mut seen_addresses = HashSet::new();
    let result = distinct_instances(8, 0, &mut seen_addresses);
    
    std::process::exit(result);
}