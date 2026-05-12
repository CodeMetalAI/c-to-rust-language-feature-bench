use std::mem::{align_of, size_of};
use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

trait TypeId {
    const ID: i32;
}

fn type_id<T: TypeId>(_: T) -> i32 {
    T::ID
}

impl TypeId for i32 {
    const ID: i32 = 1;
}
impl<'a> TypeId for &'a mut i32 {
    const ID: i32 = 2;
}
impl<'a> TypeId for &'a i32 {
    const ID: i32 = 3;
}
impl<'a> TypeId for &'a AtomicI32 {
    const ID: i32 = 4;
}
impl<'a, 'b> TypeId for &'a &'b AtomicI32 {
    const ID: i32 = 5;
}
impl TypeId for fn(i32) -> i32 {
    const ID: i32 = 6;
}
impl TypeId for [i32; 3] {
    const ID: i32 = 99;
}

fn id(v: i32) -> i32 {
    v + 1
}

fn check(cond: bool, code: i32) {
    if !cond {
        exit(code);
    }
}

fn main() {
    let cx: i32 = 9;
    let ax = AtomicI32::new(11);

    let mut a = [1, 2, 3];

    {
        let pa = &a;
        check(pa[0] == 1, 1);
        check(pa[1] == 2, 2);
        check(pa[2] == 3, 3);
    }

    check(type_id(a) == 99, 4);
    check(type_id(&mut a[0]) == 2, 5);

    check(type_id(&cx) == 3, 6);
    check(type_id(+cx) == 1, 7);
    check((+cx) == 9, 8);

    check(type_id(&ax) == 4, 9);
    check(type_id(&&ax) == 5, 10);
    let ax_val = ax.load(Ordering::SeqCst);
    check(type_id(+ax_val) == 1, 11);
    check((+ax_val) == 11, 12);

    let fp: fn(i32) -> i32 = id;
    check(type_id(id as fn(i32) -> i32) == 6, 13);
    check(fp(4) == 5, 14);
    check(id(4) == 5, 15);

    check(size_of::<[i32; 3]>() == 3 * size_of::<i32>(), 16);
    check(align_of::<i32>() == align_of::<i32>(), 17);

    exit(0);
}