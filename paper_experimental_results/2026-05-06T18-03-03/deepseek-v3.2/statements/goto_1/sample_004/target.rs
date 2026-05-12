fn main() {
    let mut acc = 0;
    let mut i = 0;
    
    // Simulate the goto by skipping the loop body on first iteration
    while i < 5 {
        if i > 0 {
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
        }
        
        acc = 15;
        i += 1;
    }
    
    if acc == 20 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}