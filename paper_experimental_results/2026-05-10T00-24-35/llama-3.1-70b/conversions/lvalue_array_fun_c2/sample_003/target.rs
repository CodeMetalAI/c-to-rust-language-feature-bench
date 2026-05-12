fn main() {
    let x: i32 = 3;
    let cx: i32 = 4;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(5);

    if std::mem::size_of_val(&x)!= std::mem::size_of::<i32>() {
        return;
    }
    if std::mem::align_of_val(&x)!= std::mem::align_of::<i32>() {
        return;
    }

    if *(&x)!= 3 {
        return;
    }

    match std::any::type_name::<i32>() {
        "i32" => (),
        _ => return,
    }
    match std::any::type_name::<std::sync::atomic::AtomicI32>() {
        "std::sync::atomic::AtomicI32" => (),
        _ => return,
    }

    match std::any::type_name::<i32>() {
        "i32" => (),
        _ => return,
    }
    match std::any::type_name::<i32>() {
        "i32" => (),
        _ => return,
    }

    let y = x;
    if y!= 3 {
        return;
    }

    let mut x_mut = x;
    x_mut += 1;
    if x_mut!= 4 {
        return;
    }

    struct S {
        m: i32,
    }
    let mut s = S { m: 7 };
    if s.m!= 7 {
        return;
    }
    s.m = 8;
    if s.m!= 8 {
        return;
    }

    println!("OK");
}