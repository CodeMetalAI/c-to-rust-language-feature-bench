fn main() {
    let expr = 0;
    let i;

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