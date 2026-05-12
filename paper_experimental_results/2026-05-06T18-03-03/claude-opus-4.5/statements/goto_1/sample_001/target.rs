fn main() {
    let mut acc = 0;
    let mut first_time_done = false;
    let mut i = 0;
    
    // Simulate goto first_time
    acc = 15;
    first_time_done = true;
    
    while i < 5 {
        if first_time_done {
            first_time_done = false;
            i += 1;
            continue;
        }
        
        acc += 1;
        if acc < 15 {
            acc = -15;
        }
        
        acc = 15;
        i += 1;
    }
    
    std::process::exit(if acc == 20 { 0 } else { 1 });
}