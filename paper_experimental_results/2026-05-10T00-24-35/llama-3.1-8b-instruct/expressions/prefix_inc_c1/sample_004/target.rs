fn main() -> i32 {
    let mut x = 5;

    let y = x + 1;
    if y!= 6 {
        return 1;
    }
    if x!= 6 {
        return 2;
    }

    let mut x = 10;
    let y = x + 1;
    let mut z = 10;
    z += 1;
    if y!= 11 {
        return 3;
    }
    if z!= 11 {
        return 4;
    }
    if z!= 11 {
        return 5;
    }

    0
}