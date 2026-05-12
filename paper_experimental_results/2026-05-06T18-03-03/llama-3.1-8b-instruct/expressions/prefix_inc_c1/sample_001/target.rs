fn main() {
    let mut x = 5;

    let y = x + 1;
    if y!= 6 {
        return 1;
    }
    if x!= 6 {
        return 2;
    }

    x = 10;
    let y = x + 1;
    let z = 10;
    let z = z + 1;
    if y!= 11 {
        return 3;
    }
    if z!= 11 {
        return 4;
    }
    if z!= 11 {
        return 5;
    }

    return 0;
}