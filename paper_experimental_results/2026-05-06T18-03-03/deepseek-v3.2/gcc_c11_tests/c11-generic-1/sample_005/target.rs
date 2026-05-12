fn check(n: bool) {
    if n {
        std::process::abort();
    }
}

fn main() {
    let mut n = 0;

    check(std::any::TypeId::of::<i32>() == std::any::TypeId::of::<i32>());
    check(n != 0);

    check(std::any::TypeId::of::<i32>() == std::any::TypeId::of::<f64>());
    check(n != 0);

    const CN: i32 = 0;
    check(std::any::TypeId::of::<i32>() == std::any::TypeId::of::<i32>());
    check(n != 0);
    check(std::any::TypeId::of::<i32>() == std::any::TypeId::of::<i32>());
    check(n != 0);

    let a: [i32; 1] = [0];
    const CA: [i32; 1] = [0];
    check(std::any::TypeId::of::<&[i32; 1]>() == std::any::TypeId::of::<&i32>());
    check(n != 0);
    check(std::any::TypeId::of::<&[i32; 1]>() == std::any::TypeId::of::<&i32>());
    check(n != 0);

    fn f() {}
    check(std::any::TypeId::of::<fn()>() == std::any::TypeId::of::<fn()>());
    check(n != 0);

    check(std::any::TypeId::of::<fn()>() == std::any::TypeId::of::<fn()>());
    check(n != 0);

    let s: i16 = 0;
    check(std::any::TypeId::of::<i16>() == std::any::TypeId::of::<i16>());
    check(n != 0);

    std::process::exit(0);
}