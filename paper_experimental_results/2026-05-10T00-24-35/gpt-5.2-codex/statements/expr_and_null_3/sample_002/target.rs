fn main() {
    let mut x = 0;

    'label: {
        break 'label;
        x = 1;
    }

    x += 1;
    let code = if x == 1 { 0 } else { 1 };
    std::process::exit(code);
}