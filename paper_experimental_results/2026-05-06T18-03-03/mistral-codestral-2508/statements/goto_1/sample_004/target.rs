fn main() {
    let mut acc = 0;
    let mut first_time = true;

    loop {
        if first_time {
            first_time = false;
        } else {
            for i in 0..5 {
                acc += 1;
                if acc < 15 {
                    acc = -15;
                }
            }
            break;
        }

        acc = 15;
    }

    std::process::exit(if acc == 20 { 0 } else { 1 });
}