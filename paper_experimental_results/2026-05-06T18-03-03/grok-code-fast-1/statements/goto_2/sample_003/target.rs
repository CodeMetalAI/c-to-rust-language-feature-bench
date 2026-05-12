fn main() {
    let n = 4;
    let j = 1;

    let mut saw_lab3 = 0;
    let mut saw_lab4 = 0;

    let mut out = 0;

    {
        let mut a = [0.0; 4];

        a[j as usize] = 4.4;
        out += 44;

        // goto lab3;

        // lab3:
        saw_lab3 = 1;
        a[j as usize] = 3.3;
        out += 33;

        // goto lab4;

        // a[j] = 5.5; // unreachable

        // lab4:
        saw_lab4 = 1;
        a[j as usize] = 6.6;
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