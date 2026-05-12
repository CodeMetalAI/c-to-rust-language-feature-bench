fn main() {
    let n: usize = 4;
    let j: usize = 1;

    let mut saw_lab3: i32 = 0;
    let mut saw_lab4: i32 = 0;

    let mut out: i32 = 0;

    {
        let mut a: Vec<f64> = vec![0.0; n];

        a[j] = 4.4;
        out += 44;

        // goto lab3;
        saw_lab3 = 1;
        a[j] = 3.3;
        out += 33;

        // goto lab4;
        saw_lab4 = 1;
        a[j] = 6.6;
        out += 66;
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