fn main() {
    let n = 4;
    let j = 1usize;
    let mut saw_lab3 = false;
    let mut saw_lab4 = false;
    let mut out = 0;
    let mut a = [0.0f64; 4];
    a[j] = 4.4;
    out += 44;
    saw_lab3 = true;
    a[j] = 3.3;
    out += 33;
    saw_lab4 = true;
    a[j] = 6.6;
    out += 66;
    if !saw_lab3 {
        std::process::exit(2);
    }
    if !saw_lab4 {
        std::process::exit(3);
    }
    if out != 143 {
        std::process::exit(4);
    }
}