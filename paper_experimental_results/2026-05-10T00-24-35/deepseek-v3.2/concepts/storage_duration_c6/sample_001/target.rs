fn distinct_instances(depth: u32, prev_addr: *mut i32) -> i32 {
    let addr = std::ptr::addr_of!(depth) as usize;
    
    if prev_addr != std::ptr::null_mut() && addr == prev_addr as usize {
        return 1;
    }
    
    if depth == 0 {
        return 0;
    }
    
    distinct_instances(depth - 1, &depth as *mut i32)
}

fn main() -> i32 {
    if distinct_instances(8, std::ptr::null_mut()) != 0 {
        return 1;
    }
    0
}