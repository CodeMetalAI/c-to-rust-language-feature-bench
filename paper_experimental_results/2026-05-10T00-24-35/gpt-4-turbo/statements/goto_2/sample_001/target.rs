fn main() {
    let n = 4;
    let j = 1;

    let mut saw_lab3 = 0;
    let mut saw_lab4 = 0;

    let mut out = 0;

    let mut a = vec![0.0; n];

    a[j] = 4.4;
    out += 44;

    // Simulating goto lab3;
    {
        saw_lab3 = 1;
        a[j] = 3.3;
        out += 33;

        // Simulating goto lab4;
        {
            saw_lab4 = 1;
            a[j] = 6.6;
            out += 66;
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
}