fn main() {
    let mut acc = 0;
    
    // Simulate the goto into the loop - first time just sets acc = 15
    acc = 15;
    
    // Then the loop runs normally for i = 0 to 4
    for _ in 0..5 {
        acc += 1;
        if acc < 15 {
            acc = -15;
        }
        acc = 15;
        // continue is implicit at end of loop
    }
    
    std::process::exit(if acc == 20 { 0 } else { 1 });
}