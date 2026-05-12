use std::process::exit;

fn main() {
    let n = 4;
    let j = 1usize;

    let mut saw_lab3 = false;
    let mut saw_lab4 = false;

    let mut out: i32 = 0;

    {
        let mut a: [f64; 4] = [0.0; 4];

        a[j] = 4.4;
        out += 44;

        // goto lab3
        saw_lab3 = true;
        a[j] = 3.3;
        out += 33;

        // goto lab4
        saw_lab4 = true;
        a[j] = 6.6;
        out += 66;

        let _ = n; // suppress unused variable warning if needed
    }

    if !saw_lab3 {
        exit(2);
    }
    if !saw_lab4 {
        exit(3);
    }
    if out != 143 {
        exit(4);
    }

    exit(0);
}