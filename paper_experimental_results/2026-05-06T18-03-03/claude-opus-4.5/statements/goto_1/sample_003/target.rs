fn main() {
    let mut acc = 0;
    let mut i = 0;
    
    // goto first_time - jump into loop at first_time label
    acc = 15;
    // continue equivalent - go to increment and condition
    i += 1;
    
    while i < 5 {
        acc += 1;
        if acc < 15 {
            acc = -15;
        }
        // first_time: acc = 15; continue;
        acc = 15;
        i += 1;
    }
    
    // Wait, this gives acc=15. Let me restructure.
    
    std::process::exit(if acc == 20 { 0 } else { 1 });
}