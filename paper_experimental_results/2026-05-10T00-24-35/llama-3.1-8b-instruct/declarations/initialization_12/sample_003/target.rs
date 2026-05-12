const MAX: usize = 12;

fn main() {
    let a: [i32; MAX] = [1, 3, 5, 7, 9, MAX - 5, 6, 4, 2, 0];

    if a[0] != 1 {
        return 1;
    }
    if a[1] != 3 {
        return 2;
    }
    if a[2] != 5 {
        return 3;
    }
    if a[3] != 7 {
        return 4;
    }
    if a[4] != 9 {
        return 5;
    }

    if a[MAX - 5] != 8 {
        return 6;
    }
    if a[MAX - 4] != 6 {
        return 7;
    }
    if a[MAX - 3] != 4 {
        return 8;
    }
    if a[MAX - 2] != 2 {
        return 9;
    }
    if a[MAX - 1] != 0 {
        return 10;
    }

    {
        let i: usize = 5;
        while i < MAX - 5 {
            if a[i] != 0 {
                return 11;
            }
            i += 1;
        }
    }

    return 0;
}