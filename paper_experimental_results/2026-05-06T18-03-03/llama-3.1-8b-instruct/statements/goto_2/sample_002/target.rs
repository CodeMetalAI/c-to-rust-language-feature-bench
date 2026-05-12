fn main() {
    let mut n = 4;
    let mut j = 1;

    let mut saw_lab3 = 0;
    let mut saw_lab4 = 0;

    let mut out = 0;

    {
        let mut a = vec![0.0; n];

        a[j] = 4.4;
        out += 44;

        goto lab3;

    lab3:
        saw_lab3 = 1;
        a[j] = 3.3;
        out += 33;

        goto lab4;

        a[j] = 5.5;

    lab4:
        saw_lab4 = 1;
        a[j] = 6.6;
        out += 66;
    }

    if !saw_lab3 {
        return 2;
    }
    if !saw_lab4 {
        return 3;
    }
    if out != 143 {
        return 4;
    }

    return 0;
}