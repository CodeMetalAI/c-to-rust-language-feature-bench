fn id(v: i32) -> i32 {
    v + 1
}

fn main() -> i32 {
    use std::mem::{align_of, size_of};

    const CX: i32 = 9;
    let ax: i32 = 11; // Rust does not have atomic types directly, simulate with regular int

    let a: [i32; 3] = [1, 2, 3];

    let pa = &a;
    if pa[0] != 1 {
        return 1;
    }
    if pa[1] != 2 {
        return 2;
    }
    if pa[2] != 3 {
        return 3;
    }

    fn type_id<T>(_: T) -> i32 {
        use std::any::TypeId;
        match TypeId::of::<T>() {
            x if x == TypeId::of::<[i32; 3]>() => 99,
            x if x == TypeId::of::<&i32>() => 2,
            x if x == TypeId::of::<&i32>() => 3,
            x if x == TypeId::of::<i32>() => 1,
            x if x == TypeId::of::<fn(i32) -> i32>() => 6,
            _ => 99,
        }
    }

    if type_id(a) != 99 {
        return 4;
    }
    if type_id(&a[0]) != 2 {
        return 5;
    }

    if type_id(&CX) != 3 {
        return 6;
    }
    if type_id(+CX) != 1 {
        return 7;
    }
    if (+CX) != 9 {
        return 8;
    }

    if type_id(ax) != 1 {
        return 9;
    }
    if type_id(&ax) != 2 {
        return 10;
    }
    if type_id(+ax) != 1 {
        return 11;
    }
    if (+ax) != 11 {
        return 12;
    }

    let fp: fn(i32) -> i32 = id;
    if type_id(id as fn(i32) -> i32) != 6 {
        return 13;
    }
    if fp(4) != 5 {
        return 14;
    }
    if id(4) != 5 {
        return 15;
    }

    if size_of::<[i32; 3]>() != 3 * size_of::<i32>() {
        return 16;
    }
    if align_of::<i32>() != align_of::<i32>() {
        return 17;
    }

    0
}