#[repr(C)]
union U {
    n: (i32,),
    ni: (i32, i32),
    nf: (i32, f64),
}

fn main() {
    let mut u = unsafe { std::mem::zeroed::<U>() };

    unsafe {
        u.nf.0 = 1;
        u.nf.1 = 3.14;

        if u.n.0 != 1 {
            std::process::exit(1);
        }
        if u.ni.0 != 1 {
            std::process::exit(1);
        }
        if u.nf.0 != 1 {
            std::process::exit(2);
        }
        if u.nf.1 != 3.14 {
            std::process::exit(3);
        }
    }

    std::process::exit(0);
}