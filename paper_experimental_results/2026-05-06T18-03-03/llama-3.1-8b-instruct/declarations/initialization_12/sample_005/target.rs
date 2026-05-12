const MAX: usize = 12;

fn main() -> i32 {
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

    if a[0]!= 1 {
        return 1;
    }
    if a[1]!= 3 {
        return 2;
    }
    if a[2]!= 5 {
        return 3;
    }
    if a[3]!= 7 {
        return 4;
    }
    if a[4]!= 9 {
        return 5;
    }

    if a[MAX - 5]!= 8 {
        return 6;
    }
    if a[MAX - 4]!= 6 {
        return 7;
    }
    if a[MAX - 3]!= 4 {
        return 8;
    }
    if a[MAX - 2]!= 2 {
        return 9;
    }
    if a[MAX - 1]!= 0 {
        return 10;
    }

    for i in 5..(MAX - 5) {
        if a[i]!= 0 {
            return 11;
        }
    }

    0
}