fn main() {
    const MAX: usize = 12;
    let a: [i32; MAX] = [1, 3, 5, 7, 9, 0, 8, 6, 4, 2, 0, 0];

    if a[0]!= 1 {
        return;
    }
    if a[1]!= 3 {
        return;
    }
    if a[2]!= 5 {
        return;
    }
    if a[3]!= 7 {
        return;
    }
    if a[4]!= 9 {
        return;
    }

    if a[MAX - 5]!= 8 {
        return;
    }
    if a[MAX - 4]!= 6 {
        return;
    }
    if a[MAX - 3]!= 4 {
        return;
    }
    if a[MAX - 2]!= 2 {
        return;
    }
    if a[MAX - 1]!= 0 {
        return;
    }

    for i in 5..MAX - 5 {
        if a[i]!= 0 {
            return;
        }
    }
}