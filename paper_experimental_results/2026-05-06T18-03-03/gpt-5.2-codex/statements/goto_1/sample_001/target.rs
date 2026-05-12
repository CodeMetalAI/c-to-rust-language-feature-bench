fn main() {
    let mut acc: i32 = 0;

    let mut i: i32 = 0;
    let mut jumped = true;

    loop {
        if jumped {
            jumped = false;
            acc = 15;
        } else {
            if i >= 5 {
                break;
            }
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
            acc = 15;
        }

        i += 1;
        if i >= 5 {
            break;
        }
    }

    let exit_code = if acc == 20 { 0 } else { 1 };
    std::process::exit(exit_code);
}