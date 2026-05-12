use std::mem;

enum TypeId {
    Int,
    IntPtr,
    ConstIntPtr,
    AtomicInt,
    AtomicIntPtr,
    FnPtr,
    ConstInt,
    Default,
}

fn type_id<T>(_x: T) -> TypeId {
    std::any::TypeId::of::<T>()
       .into_u64()
       .try_into()
       .unwrap()
       .try_into()
       .unwrap()
}

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    const CX: i32 = 9;
    let ax = std::sync::atomic::AtomicI32::new(11);

    let a = [1, 2, 3];

    let pa = &a;
    if pa[0]!= 1 {
        std::process::exit(1);
    }
    if pa[1]!= 2 {
        std::process::exit(2);
    }
    if pa[2]!= 3 {
        std::process::exit(3);
    }

    if type_id(&a)!= TypeId::Default {
        std::process::exit(4);
    }
    if type_id(&a[0])!= TypeId::IntPtr {
        std::process::exit(5);
    }

    if type_id(&CX)!= TypeId::ConstIntPtr {
        std::process::exit(6);
    }
    if type_id(CX)!= TypeId::Int {
        std::process::exit(7);
    }
    if CX!= 9 {
        std::process::exit(8);
    }

    if type_id(&ax)!= TypeId::AtomicInt {
        std::process::exit(9);
    }
    if type_id(&ax)!= TypeId::AtomicIntPtr {
        std::process::exit(10);
    }
    if type_id(ax.load(std::sync::atomic::Ordering::SeqCst))!= TypeId::Int {
        std::process::exit(11);
    }
    if ax.load(std::sync::atomic::Ordering::SeqCst)!= 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if type_id(id)!= TypeId::FnPtr {
        std::process::exit(13);
    }
    if fp(4)!= 5 {
        std::process::exit(14);
    }
    if id(4)!= 5 {
        std::process::exit(15);
    }

    if mem::size_of_val(&a)!= 3 * mem::size_of::<i32>() {
        std::process::exit(16);
    }
    if mem::align_of::<i32>()!= mem::align_of_val(&a[0]) {
        std::process::exit(17);
    }

    std::process::exit(0);
}