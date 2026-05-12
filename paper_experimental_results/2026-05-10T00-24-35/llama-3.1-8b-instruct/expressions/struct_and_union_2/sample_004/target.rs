// Define a struct to represent the C struct
#[repr(C)]
struct S {
    i: i32,
    ci: i32,
}

// Define a constant to represent the C struct
const CS: S = S { i: 3, ci: 4 };

// Define a volatile struct to represent the C struct
#[repr(C)]
struct VS {
    i: i32,
    ci: i32,
}

static mut S_STATIC: S = S { i: 1, ci: 2 };
static CS_STATIC: S = CS;
static VS_STATIC: VS = VS { i: 5, ci: 6 };

// Define a function to take a mutable reference to an i32
fn f(p: &mut i32) {
    *p = *p;
}

// Define a function to take a reference to a const i32
fn f4(p: &i32) {
    *p = *p;
}

// Define a function to take a reference to a volatile i32
fn f2(p: &mut i32) {
    *p = *p;
}

// Define a function to take a reference to a const volatile i32
fn g(p: &i32) {
    *p = *p;
}

fn main() {
    f(&mut S_STATIC.i);
    f4(&S_STATIC.ci);

    f4(&CS_STATIC.i);
    f4(&CS_STATIC.ci);

    f2(&mut VS_STATIC.i);
    g(&VS_STATIC.ci);

    if S_STATIC.i!= 1 {
        return 1;
    }
    if S_STATIC.ci!= 2 {
        return 2;
    }
    if CS_STATIC.i!= 3 {
        return 3;
    }
    if CS_STATIC.ci!= 4 {
        return 4;
    }
    if VS_STATIC.i!= 5 {
        return 5;
    }
    if VS_STATIC.ci!= 6 {
        return 6;
    }

    S_STATIC.i = 10;
    if S_STATIC.i!= 10 {
        return 7;
    }

    VS_STATIC.i = 20;
    if VS_STATIC.i!= 20 {
        return 8;
    }

    return 0;
}