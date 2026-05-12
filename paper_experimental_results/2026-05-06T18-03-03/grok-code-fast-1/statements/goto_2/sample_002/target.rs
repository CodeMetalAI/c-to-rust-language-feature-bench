fn main() {
    let n = 4;
    let j = 1;
    let mut saw_lab3 = false;
    let mut saw_lab4 = false;
    let mut out = 0;

    {
        let mut _a: [f64; 4] = [0.0; 4];
        _a[j] = 4.4;
        out += 44;
        // goto lab3 equivalent: jump to lab3 code
        saw_lab3 = true;
        _a[j] = 3.3;
        out += 33;
        // goto lab4 equivalent: jump to lab4 code
        saw_lab4 = true;
        _a[j] = 6.6;
        out += 66;
    }

    if !saw_lab3 {
        std::process::exit(2);
    }
    if !saw_lab4 {
        std::process::exit(3);
    }
    if out != 143 {
        std::process::exit(4);
    }
    std::process::exit(0);
}