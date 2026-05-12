use std::sync::atomic::AtomicI32;

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    const CX: i32 = 9;
    let ax = AtomicI32::new(11);

    let a = [1, 2, 3];

    let pa = &a[0];
    if pa != &1 {
        return;
    }
    if a[1] != 2 {
        return;
    }
    if a[2] != 3 {
        return;
    }

    // TYPE_ID checks - we simulate them with type comparisons
    // TYPE_ID(a) -> 99 (array type not in list)
    // TYPE_ID(&a[0]) -> 2 (int*)
    if !(std::any::type_name_of_val(&a).contains("[")) {
        return;
    }
    if !(std::any::type_name_of_val(pa).contains("&i32")) {
        return;
    }

    // TYPE_ID(&cx) -> 3 (const int*)
    let cx_ptr = &CX;
    if !(std::any::type_name_of_val(cx_ptr).contains("&i32")) {
        return;
    }
    // TYPE_ID(+cx) -> 1 (int)
    let cx_val = CX;
    if !(std::any::type_name_of_val(&cx_val).contains("i32")) {
        return;
    }
    if cx_val != 9 {
        return;
    }

    // TYPE_ID(ax) -> 4 (_Atomic(int))
    if !(std::any::type_name_of_val(&ax).contains("AtomicI32")) {
        return;
    }
    // TYPE_ID(&ax) -> 5 (_Atomic(int)*)
    let ax_ptr = &ax;
    if !(std::any::type_name_of_val(ax_ptr).contains("&AtomicI32")) {
        return;
    }
    // TYPE_ID(+ax) -> 1 (int)
    let ax_val = ax.load(std::sync::atomic::Ordering::SeqCst);
    if !(std::any::type_name_of_val(&ax_val).contains("i32")) {
        return;
    }
    if ax_val != 11 {
        return;
    }

    // TYPE_ID(id) -> 6 (int (*)(int))
    let fp: fn(i32) -> i32 = id;
    if !(std::any::type_name_of_val(&fp).contains("fn")) {
        return;
    }
    if fp(4) != 5 {
        return;
    }
    if id(4) != 5 {
        return;
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        return;
    }
    if std::mem::align_of::<i32>() != std::mem::align_of_val(&a[0]) {
        return;
    }

    std::process::exit(0);
}