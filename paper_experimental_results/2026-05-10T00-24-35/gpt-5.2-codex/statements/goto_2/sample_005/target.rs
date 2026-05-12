fn main() {
    let n: usize = 4;
    let j: usize = 1;

    let mut saw_lab3: i32 = 0;
    let mut saw_lab4: i32 = 0;

    let mut out: i32 = 0;

    {
        let mut a = vec![0.0f64; n];

        let mut state = 0;
        loop {
            match state {
                0 => {
                    a[j] = 4.4;
                    out += 44;
                    state = 1;
                }
                1 => {
                    saw_lab3 = 1;
                    a[j] = 3.3;
                    out += 33;
                    state = 2;
                }
                2 => {
                    saw_lab4 = 1;
                    a[j] = 6.6;
                    out += 66;
                    break;
                }
                _ => break,
            }
        }
    }

    if saw_lab3 == 0 {
        std::process::exit(2);
    }
    if saw_lab4 == 0 {
        std::process::exit(3);
    }
    if out != 143 {
        std::process::exit(4);
    }

    std::process::exit(0);
}