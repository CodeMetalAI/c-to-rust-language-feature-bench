#[repr(C)]
union U {
    n: Alltypes,
    ni: Ni,
    nf: Nf,
}

#[repr(C)]
struct Alltypes {
    alltypes: i32,
}

#[repr(C)]
struct Ni {
    type_: i32,
    intnode: i32,
}

#[repr(C)]
struct Nf {
    type_: i32,
    doublenode: f64,
}

fn main() {
    let mut u: U = unsafe { std::mem::zeroed() };

    unsafe { u.nf.type_ = 1; }

    unsafe { u.nf.doublenode = 3.14; }

    if unsafe { u.n.alltypes } != 1 {
        std::process::exit(1);
    }
    if unsafe { u.ni.type_ } != 1 {
        std::process::exit(1);
    }
    if unsafe { u.nf.type_ } != 1 {
        std::process::exit(2);
    }
    if unsafe { u.nf.doublenode } != 3.14 {
        std::process::exit(3);
    }

    std::process::exit(0);
}