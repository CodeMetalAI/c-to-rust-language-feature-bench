fn main() {
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
    if ((a < b) == (c < d))!= 1 {
        return 1;
    }

    let a = 2;
    let b = 1;
    let c = 4;
    let d = 3;
    if ((a < b) == (c < d))!= 1 {
        return 2;
    }

    let a = 1;
    let b = 2;
    let c = 4;
    let d = 3;
    if ((a < b) == (c < d))!= 0 {
        return 3;
    }

    let a = 2;
    let b = 1;
    let c = 3;
    let d = 4;
    if ((a < b) == (c < d))!= 0 {
        return 4;
    }

    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
    if ((a < b) == (c < d))!= 1 {
        return 5;
    }

    return 0;
}