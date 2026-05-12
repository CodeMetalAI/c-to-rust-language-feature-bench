fn main() {
    let expr = 0;

    match expr {
        0 => {
            let i = 17;
            if i == 17 {
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        }
        _ => unreachable!(),
    }
}