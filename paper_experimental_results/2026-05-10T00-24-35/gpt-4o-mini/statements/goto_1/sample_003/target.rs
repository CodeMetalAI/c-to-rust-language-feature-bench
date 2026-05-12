fn main() {
    let mut acc = 0;

    // Simulating the goto by initializing acc to 15 before entering the loop
    acc = 15;

    for _ in 0..5 {
        acc += 1;
        if acc < 15 {
            acc = -15;
        }
        // The loop continues, we don't need to do anything special here.
    }

    // Checking the return condition
    std::process::exit(if acc == 20 { 0 } else { 1 });
}