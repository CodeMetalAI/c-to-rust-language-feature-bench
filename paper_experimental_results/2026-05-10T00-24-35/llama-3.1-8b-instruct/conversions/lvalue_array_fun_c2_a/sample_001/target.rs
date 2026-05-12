#![allow(non_snake_case)]

fn id(x: i32) -> i32 {
    x + 1
}

fn main() {
    const CX: i32 = 9;
    let mut AX: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(11);

    let A: [i32; 3] = [1, 2, 3];

    let PA: *const i32 = &A[0];
    if *PA!= 1 {
        panic!("PA[0]!= 1");
    }
    if *PA.offset(1)!= 2 {
        panic!("PA[1]!= 2");
    }
    if *PA.offset(2)!= 3 {
        panic!("PA[2]!= 3");
    }

    let TYPE_ID_A: u8 = TYPE_ID(&A);
    if TYPE_ID_A!= 99 {
        panic!("TYPE_ID(A)!= 99");
    }

    let TYPE_ID_PA: u8 = TYPE_ID(PA);
    if TYPE_ID_PA!= 2 {
        panic!("TYPE_ID(PA)!= 2");
    }

    let TYPE_ID_CX: u8 = TYPE_ID(&CX);
    if TYPE_ID_CX!= 3 {
        panic!("TYPE_ID(CX)!= 3");
    }
    if TYPE_ID(+CX)!= 1 {
        panic!("TYPE_ID(+CX)!= 1");
    }
    if +CX!= 9 {
        panic!("+CX!= 9");
    }

    let TYPE_ID_AX: u8 = TYPE_ID(AX);
    if TYPE_ID_AX!= 4 {
        panic!("TYPE_ID(AX)!= 4");
    }
    let TYPE_ID_PA_AX: u8 = TYPE_ID(&AX);
    if TYPE_ID_PA_AX!= 5 {
        panic!("TYPE_ID(&AX)!= 5");
    }
    if TYPE_ID(+AX)!= 1 {
        panic!("TYPE_ID(+AX)!= 1");
    }
    if +AX.load(std::sync::atomic::Ordering::SeqCst)!= 11 {
        panic!("+AX!= 11");
    }

    let FP: fn(i32) -> i32 = id;
    let TYPE_ID_FP: u8 = TYPE_ID(FP);
    if TYPE_ID_FP!= 6 {
        panic!("TYPE_ID(id)!= 6");
    }
    if FP(4)!= 5 {
        panic!("FP(4)!= 5");
    }
    if id(4)!= 5 {
        panic!("id(4)!= 5");
    }

    if std::mem::size_of::<[i32; 3]>()!= 3 * std::mem::size_of::<i32>() {
        panic!("std::mem::size_of<[i32; 3]>()!= 3 * std::mem::size_of::<i32>()");
    }
    if std::mem::align_of::<i32>()!= std::mem::align_of::<[i32; 3]>() {
        panic!("std::mem::align_of::<i32>()!= std::mem::align_of::<[i32; 3]>()");
    }

    println!("0");
}

fn TYPE_ID<T>(x: T) -> u8 {
    match std::any::type_id_of::<T>() {
        std::any::TypeId::of::<i32>() => 1,
        std::any::TypeId::of::<*const i32>() => 2,
        std::any::TypeId::of::<&'static i32>() => 3,
        std::any::TypeId::of::<std::sync::atomic::AtomicI32>() => 4,
        std::any::TypeId::of::<&'static std::sync::atomic::AtomicI32>() => 5,
        std::any::TypeId::of::<fn(i32) -> i32>() => 6,
        std::any::TypeId::of::<i32>() => 7,
        _ => 99,
    }
}