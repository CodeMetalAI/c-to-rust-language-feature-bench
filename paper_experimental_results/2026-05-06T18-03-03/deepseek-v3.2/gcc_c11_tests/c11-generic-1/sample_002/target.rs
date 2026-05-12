fn check(n: bool) {
    if n {
        std::process::abort();
    }
}

fn main() {
    let mut n = 0;

    check(std::mem::discriminant(&n) == std::mem::discriminant(&0i32));
    check(n != 0);

    check(std::mem::discriminant(&n) == std::mem::discriminant(&0.0f64));
    check(n != 0);

    let cn = 0;
    check(std::mem::discriminant(&cn) == std::mem::discriminant(&0i32));
    check(n != 0);
    check(std::mem::discriminant(&cn) == std::mem::discriminant(&0i32));
    check(n != 0);

    let a = [0i32; 1];
    let ca = [0i32; 1];
    check(std::any::TypeId::of::<[i32; 1]>() == std::any::TypeId::of::<[i32; 1]>());
    check(n != 0);
    check(std::any::TypeId::of::<[i32; 1]>() == std::any::TypeId::of::<[i32; 1]>());
    check(n != 0);

    fn f() {}
    check(std::any::TypeId::of::<fn()>() == std::any::TypeId::of::<fn()>());
    check(n != 0);

    check(std::any::TypeId::of::<fn()>() == std::any::TypeId::of::<fn()>());
    check(n != 0);

    let s: i16 = 0;
    check(std::mem::discriminant(&s) == std::mem::discriminant(&0i16));
    check(n != 0);

    std::process::exit(0);
}