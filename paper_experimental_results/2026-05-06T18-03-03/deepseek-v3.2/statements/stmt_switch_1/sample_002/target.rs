fn main() {
    let expr = 0;
    let i = match expr {
        0 => {
            let i = 17;
            if i == 17 { 0 } else { 1 }
        }
        _ => {
            let _i = 4;
            1
        }
    };
    std::process::exit(i);
}