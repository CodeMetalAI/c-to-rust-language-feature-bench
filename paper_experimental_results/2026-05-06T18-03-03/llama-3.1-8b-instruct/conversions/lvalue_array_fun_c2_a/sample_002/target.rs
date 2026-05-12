fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx: i32 = 9;
    let mut ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(11);

    let a = [1, 2, 3];

    let pa = &a;
    if pa[0] != 1 {
        panic!("pa[0] != 1");
    }
    if pa[1] != 2 {
        panic!("pa[1] != 2");
    }
    if pa[2] != 3 {
        panic!("pa[2] != 3");
    }

    if std::any::type_id(&a) != std::any::type_id(&[1, 2, 3]) {
        panic!("std::any::type_id(&a) != std::any::type_id(&[1, 2, 3])");
    }
    if std::any::type_id(&a[0]) != std::any::type_id(&1) {
        panic!("std::any::type_id(&a[0]) != std::any::type_id(&1)");
    }

    if std::any::type_id(&cx) != std::any::type_id(&9) {
        panic!("std::any::type_id(&cx) != std::any::type_id(&9)");
    }
    if std::any::type_id(+cx) != std::any::type_id(9) {
        panic!("std::any::type_id(+cx) != std::any::type_id(9)");
    }
    if +cx != 9 {
        panic!("+cx != 9");
    }

    if std::any::type_id(ax) != std::any::type_id(std::sync::atomic::AtomicI32::new(11)) {
        panic!("std::any::type_id(ax) != std::any::type_id(std::sync::atomic::AtomicI32::new(11))");
    }
    if std::any::type_id(&ax) != std::any::type_id(&std::sync::atomic::AtomicI32::new(11)) {
        panic!("std::any::type_id(&ax) != std::any::type_id(&std::sync::atomic::AtomicI32::new(11))");
    }
    if std::any::type_id(+ax) != std::any::type_id(11) {
        panic!("std::any::type_id(+ax) != std::any::type_id(11)");
    }
    if +ax != 11 {
        panic!("+ax != 11");
    }

    let fp: fn(i32) -> i32 = id;
    if std::any::type_id(id) != std::any::type_id(|x| x + 1) {
        panic!("std::any::type_id(id) != std::any::type_id(|x| x + 1)");
    }
    if fp(4) != 5 {
        panic!("fp(4) != 5");
    }
    if id(4) != 5 {
        panic!("id(4) != 5");
    }

    if std::mem::size_of::<[i32; 3]>() != 3 * std::mem::size_of::<i32>() {
        panic!("std::mem::size_of::<[i32; 3]>() != 3 * std::mem::size_of::<i32>()");
    }
    if std::mem::align_of::<i32>() != std::mem::align_of::<[i32; 3]>() {
        panic!("std::mem::align_of::<i32>() != std::mem::align_of::<[i32; 3]>()");
    }

    assert_eq!(0, 0);
}