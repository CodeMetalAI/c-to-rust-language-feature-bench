fn main() {
    let n: usize = 4;
    let j: usize = 1;

    let mut saw_lab3 = false;
    let mut saw_lab4 = false;

    let mut out: i32 = 0;

    {
        let mut a = vec![0f64; n];

        let mut pc = 0;
        loop {
            match pc {
                0 => {
                    a[j] = 4.4;
                    out += 44;
                    pc = 1; // goto lab3
                }
                1 => {
                    saw_lab3 = true;
                    a[j] = 3.3;
                    out += 33;
                    pc = 2; // goto lab4
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