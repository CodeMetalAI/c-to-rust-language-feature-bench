fn main() {
    let mut x = 0;

    'block: {
        break 'block;
        x = 1;
    }

    x += 1;
    let code = if x == 1 { 0 } else { 1 };
    std::process::exit(code);
}