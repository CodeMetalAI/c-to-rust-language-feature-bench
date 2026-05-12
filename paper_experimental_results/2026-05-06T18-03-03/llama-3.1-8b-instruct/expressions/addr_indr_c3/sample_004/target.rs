fn main() {
    let x = 7;
    let p = &x;

    if &x!= p {
        return 1;
    }

    let a = [10, 20, 30];

    if &a[0]!= &a[0] {
        return 3;
    }
    if &a[1]!= &a[1] {
        return 4;
    }
    if &a[2]!= &a[2] {
        return 5;
    }

    if a[1]!= 20 {
        return 6;
    }

    return 0;
}