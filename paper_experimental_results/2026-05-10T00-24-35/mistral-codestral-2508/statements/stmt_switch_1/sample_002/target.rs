fn main() {
    let expr = 0;

    let i = match expr {
        0 => {
            let i = 4;
            let i = 17;
            i
        }
        _ => unreachable!(),
    };

    if i == 17 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}