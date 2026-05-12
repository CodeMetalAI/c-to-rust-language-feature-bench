fn main() -> i32 {
    let mut x = 5;
    let y = {
        x -= 1;
        x
    };
    if y != 4 {
        return 1;
    }
    if x != 4 {
        return 2;
    }

    x = 10;
    let y = {
        x -= 1;
        x
    };
    let mut z = 10;
    z -= 1;
    if y != 9 {
        return 3;
    }
    if x != 9 {
        return 4;
    }
    if z != 9 {
        return 5;
    }

    return 0;
}