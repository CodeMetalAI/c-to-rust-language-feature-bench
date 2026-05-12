fn main() {
    let a = [1, 2];
    let b = [3, 4, 5];

    if a.len() != 2 {
        return 1;
    }
    if b.len() != 3 {
        return 2;
    }

    if sum(&a) != 3 {
        return 3;
    }
    if sum(&b) != 12 {
        return 4;
    }

    if a[1] != 2 {
        return 5;
    }
    if b[2] != 5 {
        return 6;
    }

    return 0;
}

fn sum(arr: &Vec<i32>) -> i32 {
    let mut s = 0;
    for i in 0..arr.len() {
        s += arr[i];
    }
    s
}