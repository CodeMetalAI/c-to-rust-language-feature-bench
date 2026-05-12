struct Pair {
    a: i32,
    b: i32,
}

fn main() {
    let p: &[i32] = &[10, 20, 30];
    if p[0] != 10 {
        return;
    }
    if p[1] != 20 {
        return;
    }
    if p[2] != 30 {
        return;
    }

    let q = Pair { a: 7, b: 11 };
    if q.a != 7 {
        return;
    }
    if q.b != -11 {
        return;
    }

    let arr: &[i32] = &[1, 2, 3];
    if arr[1] != 2 {
        return;
    }

    return;
}