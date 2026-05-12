// Define a type alias for unsigned long
type Uptr = u64;

// Define the Outer struct
#[derive(Debug)]
struct Outer {
    base: i32,
    a: i32,
    b: i32,
    deep: i32,
    c: i32,
    d: i32,
    tail_x: i32,
    tail_y: i32,
    tail_p: i32,
    tail_q: i32,
}

// Define a function to check designated initialization
fn check_designated_init() -> i32 {
    let mut o = Outer {
        base: 10,
        a: 1,
        b: 2,
        deep: 99,
        c: 3,
        d: 4,
        tail_x: 7,
        tail_y: 8,
        tail_p: 0,
        tail_q: 0,
    };

    if o.base!= 10 {
        return 1;
    }
    if o.a!= 1 {
        return 2;
    }
    if o.b!= 2 {
        return 3;
    }
    if o.deep!= 99 {
        return 4;
    }
    if o.c!= 3 {
        return 5;
    }
    if o.d!= 4 {
        return 6;
    }
    if o.tail_x!= 7 {
        return 7;
    }
    if o.tail_y!= 8 {
        return 8;
    }

    0
}

// Define a function to check union aliasing via flattened names
fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        a: 0,
        b: 0,
        deep: 0,
        c: 0,
        d: 0,
        tail_x: 0,
        tail_y: 0,
        tail_p: 0,
        tail_q: 0,
    };

    o.deep = 0x11223344;

    if o.x!= 0x11223344 {
        return 20;
    }

    o.x = 5;
    o.y = 6;
    if o.x!= 5 {
        return 21;
    }
    if o.y!= 6 {
        return 22;
    }

    o.tail_p = 40;
    o.tail_q = 41;
    if o.tail_p!= 40 {
        return 23;
    }
    if o.tail_q!= 41 {
        return 24;
    }

    o.tail_y = -9;
    if o.tail_y!= -9 {
        return 25;
    }

    0
}

// Define a function to check addressability
fn check_addressability() -> i32 {
    let mut o = Outer {
        base: 0,
        a: 0,
        b: 0,
        deep: 0,
        c: 0,
        d: 0,
        tail_x: 0,
        tail_y: 0,
        tail_p: 0,
        tail_q: 0,
    };

    let pa: Uptr = o.a as Uptr;
    let pb: Uptr = o.b as Uptr;
    let pdeep: Uptr = o.deep as Uptr;
    let ptx: Uptr = o.tail_x as Uptr;
    let pty: Uptr = o.tail_y as Uptr;

    if pa == 0 || pb == 0 || pdeep == 0 || ptx == 0 || pty == 0 {
        return 30;
    }
    if pa == pb {
        return 31;
    }
    if ptx == pty {
        return 32;
    }

    0
}

fn main() {
    let r = check_designated_init();
    if r!= 0 {
        return;
    }

    r = check_union_aliasing_via_flattened_names();
    if r!= 0 {
        return;
    }

    r = check_addressability();
    if r!= 0 {
        return;
    }

    println!("All tests passed.");
}