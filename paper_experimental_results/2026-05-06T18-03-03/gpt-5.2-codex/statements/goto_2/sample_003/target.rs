use std::process::exit;

fn main() {
    let n: usize = 4;
    let j: usize = 1;

    let mut saw_lab3 = false;
    let mut saw_lab4 = false;

    let mut out: i32 = 0;

    {
        let mut a = vec![0.0f64; n];

        let mut state = 0;
        loop {
            match state {
                0 => {
                    a[j] = 4.4;
                    out += 44;
                    state = 1; // goto lab3
                }
                1 => {
                    saw_lab3 = true;
                    a[j] = 3.3;
                    out += 33;
                    state = 2; // goto lab4
                }
                2 => {
                    saw_lab4 = true;
                    a[j] = 6.6;
                    out += 66;
                    break;
                }
                _ => break,
            }
        }
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