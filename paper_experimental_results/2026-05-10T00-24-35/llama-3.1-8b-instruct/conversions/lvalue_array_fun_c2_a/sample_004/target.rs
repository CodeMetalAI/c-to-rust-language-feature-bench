fn main() {
    let cx: i32 = 9;
    let mut ax = 11;

    let a = [1, 2, 3];

    let pa = &a[0];
    if *pa != 1 {
        return 1;
    }
    if a[1] != 2 {
        return 2;
    }
    if a[2] != 3 {
        return 3;
    }

    if a.type_id() != 99 {
        return 4;
    }
    if a.as_ptr().type_id() != 2 {
        return 5;
    }

    if &cx.type_id() != 3 {
        return 6;
    }
    if cx.type_id() != 1 {
        return 7;
    }
    if cx != 9 {
        return 8;
    }

    if ax.type_id() != 4 {
        return 9;
    }
    if &ax.type_id() != 5 {
        return 10;
    }
    if ax.type_id() != 1 {
        return 11;
    }
    if ax != 11 {
        return 12;
    }

    let fp = id;
    if fp.type_id() != 6 {
        return 13;
    }
    if fp(4) != 5 {
        return 14;
    }
    if id(4) != 5 {
        return 15;
    }

    if a.len() != 3 {
        return 16;
    }
    if std::mem::align_of::<i32>() != std::mem::align_of!(i32) {
        return 17;
    }

    return 0;
}

fn id(x: i32) -> i32 {
    return x + 1;
}