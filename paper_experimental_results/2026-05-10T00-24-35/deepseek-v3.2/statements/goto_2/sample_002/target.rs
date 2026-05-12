fn main() {
    let mut n = 4;
    let mut j = 1;
    let mut saw_lab3 = false;
    let mut saw_lab4 = false;
    let mut out = 0.0;

    let mut a = [0.0; 5];

    loop {
        a[j] = 4.4;
        out += 44.0;
        if saw_lab3 {
            break;
        }

        a[j] = 3.৩;
        out += 33.0;
        if saw_lab4 {
            break;
        }
        a[j] = 5.5;
        out += 55.0;
        saw_lab4 = true;
        continue;
    }
    a[j] = 6.6;
    out += 66.0;
    saw_lab3 = true;
    continue;
}

if !saw_lab3 {
    return;
}
if ! saw_lab4 {
    return;
}
if out != 143.0 {
    return;
}

return;
}