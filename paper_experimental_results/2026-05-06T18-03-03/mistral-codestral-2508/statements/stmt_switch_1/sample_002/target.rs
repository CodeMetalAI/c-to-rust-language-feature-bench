fn main() {
    let expr = 0;

    let i = match expr {
        0 => {
            let mut i = 4;
            i = 17;
            i
        }
        _ => unreachable!(),
    };

    std::process::exit(if i == 17 { 0 } else { 1 });
}