fn main() {
    let n = 4;
    let j = 1;

    let mut saw_lab3 = 0;
    let mut saw_lab4 = -1; // Will be set to 1 later, but initialize to avoid UB
    let mut out = 0;

    // Simulate the block with goto behavior using explicit state tracking
    let mut state = 0;
    let mut a = vec![0.0; n];

    loop {
        match state {
            0 => {
                a[j] = 4.4;
                out += 44;
                state = 1;
                continue;
            }
            1 => {
                saw_lab3 = 1;
                a[j] = 3.3;
                out += 33;
                state = 2;
                continue;
            }
            2 => {
                // This corresponds to the goto lab4; skipping the a[j] = 5.5;
                saw_lab4 = 1;
                a[j] = 6.6;
                out += 66;
                break;
            }
            _ => unreachable!(),
        }
    }

    if saw_lab3 == 0 {
        std::process::exit(2);
    }
    if saw_lab4 == 0 {
        std::process::exit(3);
    }
    if out != 143 {
        std::process::exit(4);
    }

    std::process::exit(0);
}