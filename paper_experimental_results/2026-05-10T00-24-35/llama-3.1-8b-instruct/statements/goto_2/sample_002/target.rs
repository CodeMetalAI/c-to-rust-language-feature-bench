fn main() {
    let n = 4;
    let mut j = 1;

    let mut saw_lab3 = false;
    let mut saw_lab4 = false;

    let mut out = 0;

    {
        let mut a: [f64; 4] = [0.0; 4];

        a[j] = 4.4;
        out += 44;

        goto lab3;

        loop {}
        lab3:
        saw_lab3 = true;
        a[j] = 3.3;
        out += 33;

        goto lab4;

        a[j] = 5.5;

        loop {}
        lab4:
        saw_lab4 = true;
        a[j] = 6.6;
        out += 66;
    }

    if!saw_lab3 {
        return 2;
    }
    if!saw_lab4 {
        return 3;
    }
    if out!= 143 {
        return 4;
    }

    return 0;
}