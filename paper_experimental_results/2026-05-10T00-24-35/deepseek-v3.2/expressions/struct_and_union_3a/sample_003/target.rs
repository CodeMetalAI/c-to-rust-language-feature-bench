use std::mem::ManuallyDrop;

union U {
    all: ManuallyDrop<All>,
    ni: ManuallyDrop<Ni>,
    nf: ManuallyDrop<Nf>,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct All {
    alltypes: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Ni {
    type_: i32,
    intnode: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Nf {
    type_: i32,
    doublenode: f64,
}

fn main() {
    let mut u = U {
        all: ManuallyDrop::new(All { alltypes: 0 }),
    };

    unsafe {
        u.nf.type_ = 1;
        u.nf.doublenode = 3.14;
    }

    let success = unsafe {
        if u.all.alltypes != 1 {
            false
        } else if u.ni.type_ != 1 {
            false
        } else if u.nf.type_ != 1 {
            false
        } else if u.nf.doublenode != 3.14 {
            false
        } else {
            true
        }
    };

    std::process::exit(if success { 0 } else { 1 });
}