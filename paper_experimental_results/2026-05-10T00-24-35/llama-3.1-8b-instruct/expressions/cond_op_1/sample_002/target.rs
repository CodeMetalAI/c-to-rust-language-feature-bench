fn f<T>(_p: T)
where
    T: std::marker::Copy,
{
    _p
}

fn f1<T>(_p: T)
where
    T: std::marker::Copy,
{
    _p
}

fn f2<T>(_p: T)
where
    T: std::marker::Copy + std::marker::Sized + std::sync::atomic::AtomicIsize,
{
    _p
}

fn f3<T>(_p: T)
where
    T: std::marker::Copy + std::marker::Sized + std::sync::atomic::AtomicIsize,
{
    _p
}

fn f4<T>(_p: T)
where
    T: std::marker::Copy,
{
    _p
}

fn main() {
    let c_vp = std::ptr::null::<()>(); // equivalent to const void *c_vp;
    let vp = std::ptr::null::<()>(); // equivalent to void *vp;
    let c_ip = &0; // equivalent to const int *c_ip;
    let v_ip = &0; // equivalent to volatile int *v_ip;
    let ip = &0; // equivalent to int *ip;
    let c_cp = std::ptr::null::<()>(); // equivalent to const char *c_cp;

    f(1? c_vp : c_ip);
    f(1? c_ip : c_vp);

    f2(1? v_ip : &0);
    f2(1? &0 : v_ip);

    f3(1? c_ip : v_ip);
    f3(1? v_ip : c_ip);

    f(1? vp : c_cp);
    f(1? c_cp : vp);

    f4(1? ip : c_ip);
    f4(1? c_ip : ip);

    f1(1? vp : ip);
    f1(1? ip : vp);

    std::process::exit(0);
}