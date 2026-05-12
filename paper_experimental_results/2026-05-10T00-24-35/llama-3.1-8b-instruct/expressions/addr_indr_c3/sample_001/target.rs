fn main() {
    let mut x: i32 = 7;
    let p = &x;

    if &x!= p {
        return 1;
    }

    if &(*p)!= p {
        return 2;
    }

    let mut a: [i32; 3] = [10, 20, 30];

    if &a[0]!= a {
        return 3;
    }
    if &a[1]!= a {
        return 4;
    }
    if &a[2]!= a {
        return 5;
    }

    if a[1]!= 20 {
        return 6;
    }

    return 0;
}