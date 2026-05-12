#![allow(unused)]

fn id(x: i32) -> i32 {
    x + 1
}

fn main() {
    const CX: i32 = 9;
    let mut AX: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(11);

    let A: [i32; 3] = [1, 2, 3];

    let PA: &i32 = &A[0];
    if *PA != 1 {
        return 1;
    }
    if *PA.offset(1) != 2 {
        return 2;
    }
    if *PA.offset(2) != 3 {
        return 3;
    }

    if std::mem::size_of_val(&A) != 3 * std::mem::size_of::<i32>() {
        return 4;
    }
    if std::mem::size_of_val(&A[0]) != std::mem::size_of::<i32>() {
        return 5;
    }

    if std::mem::size_of_val(&CX) != std::mem::size_of::<i32>() {
        return 6;
    }
    if CX != 9 {
        return 7;
    }
    if +CX != 9 {
        return 8;
    }

    let FP: fn(i32) -> i32 = id;
    if std::mem::size_of_val(&FP) != std::mem::size_of::<fn(i32)>() {
        return 13;
    }
    if FP(4) != 5 {
        return 14;
    }
    if id(4) != 5 {
        return 15;
    }

    if std::mem::size_of_val(&A) != 3 * std::mem::size_of::<i32>() {
        return 16;
    }
    if std::mem::align_of_val::<i32>() != std::mem::align_of::<i32>() {
        return 17;
    }

    0
}