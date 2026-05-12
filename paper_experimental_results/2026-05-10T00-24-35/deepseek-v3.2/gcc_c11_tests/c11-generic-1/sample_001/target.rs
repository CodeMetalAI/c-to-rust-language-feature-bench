fn abort() -> ! {
    std::process::abort();
}

fn check(n: i32) {
    if n != 0 {
        abort();
    }
}

fn f() {}

fn main() {
    let mut n = 0;

    check({
        let mut temp = n;
        match () {
            _ if std::mem::discriminant(&(temp as i32)) == std::mem::discriminant(&0i32) => 0,
            _ => {
                temp += 1;
                0
            }
        }
    });
    check(n);

    check({
        match () {
            _ if std::mem::discriminant(&n) == std::mem::discriminant(&0.0f64) => n,
            _ => 0,
        }
    });
    check(n);

    let cn = 0;
    check({
        match () {
            _ if std::mem::discriminant(&cn) == std::mem::discriminant(&0i32) => 0,
            _ => {
                n += 1;
                0
            }
        }
    });
    check(n);
    check({
        let const_n = n as i32;
        match () {
            _ if std::mem::discriminant(&const_n) == std::mem::discriminant(&0i32) => 0,
            _ => {
                n += 1;
                0
            }
        }
    });
    check(n);

    let a = [0i32; 1];
    let ca = [0i32; 1];
    check({
        match () {
            _ if std::any::TypeId::of::<&[i32; 1]>() == std::any::TypeId::of::<&[i32; 1]>() => 0,
            _ => {
                n += 1;
                0
            }
        }
    });
    check(n);
    check({
        match () {
            _ if std::any::TypeId::of::<&[i32; 1]>() == std::any::TypeId::of::<&[i32; 1]>() => 0,
            _ => {
                n += 1;
                0
            }
        }
    });
    check(n);

    check({
        match () {
            _ if std::any::TypeId::of::<fn()>() == std::any::TypeId::of::<fn()>() => 0,
            _ => {
                n += 1;
                0
            }
        }
    });
    check(n);

    check({
        let abort_ptr: fn() -> ! = abort;
        match () {
            _ if std::any::TypeId::of::<fn() -> !>() == std::any::TypeId::of::<fn() -> !>() => 0,
            _ => {
                n += 1;
                0
            }
        }
    });
    check(n);

    let s: i16 = 0;
    check({
        match () {
            _ if std::mem::discriminant(&s) == std::mem::discriminant(&0i16) => 0,
            _ => {
                n += 1;
                0
            }
        }
    });
    check(n);

    std::process::exit(0);
}