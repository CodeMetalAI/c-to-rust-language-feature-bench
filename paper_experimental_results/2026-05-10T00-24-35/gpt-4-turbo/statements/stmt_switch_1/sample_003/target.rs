fn main() {
    let mut expr = 0;
    let mut i = 4; // Initialize i before the switch-case equivalent

    match expr {
        0 => {
            i = 17;
            if i == 17 {
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        },
        _ => {}
    }
}