fn main() {
    const MAX: usize = 12;
    let mut a = [0; MAX];
    a[0] = 1;
    a[1] = 3;
    a[2] = 5;
    a[3] = 7;
    a[4] = 9;
    a[MAX - 5] = 8;
    a[MAX - 4] = 6;
    a[MAX - 3] = 4;
    a[MAX - 2] = 2;
    a[MAX - 1] = 0;

    if a[0] != 1 {
        return;
    }
    if a[1] != 3 {
        return;
    }
    if a[2] != 5 {
        return;
    }
    if a[3] != 7 {
        return;
    }
    if a[4] != 9 {
        return;
    }

    if a[MAX - 5] != 8 {
        return;
    }
    if a[MAX - 4] != 6 {
        return;
    }
    if a[MAX - 3] != 4 {
        return;
    }
    if a[MAX - 2] != 2 {
        return;
    }
    if a[MAX - 1] != 0 {
        return;
    }

    let mut i = 5;
    while i < MAX - 5 {
        if a[i] != 0 {
            return;
        }
        i += 1;
    }

    std::process::exit(0);
}