#[allow(unused_imports)]
use std::{
    fmt::{Debug, Display},
    io::{BufRead, Write},
};

#[derive(Debug)]
struct S1 {
    x: i32,
}

#[derive(Debug)]
struct S2 {
    x: i32,
}

fn takes_t1(v: S1) -> i32 {
    v.x + 1
}

fn takes_s1(v: S1) -> i32 {
    v.x + 2
}

fn takes_tp1(p: &mut S1) -> i32 {
    p.x += 3;
    p.x
}

fn takes_int(v: i32) -> i32 {
    v + 4
}

fn takes_t2(v: S2) -> i32 {
    v.x + 5
}

fn main() {
    let mut a = S1 { x: 10 };
    let mut b = S1 { x: 20 };

    if takes_t1(a) != 11 {
        panic!("Expected takes_t1 to return 11");
    }

    if takes_s1(a) != 12 {
        panic!("Expected takes_s1 to return 12");
    }

    if takes_t1(b.clone()) != 21 {
        panic!("Expected takes_t1 to return 21");
    }

    if takes_s1(b.clone()) != 22 {
        panic!("Expected takes_s1 to return 22");
    }

    let mut p = &mut a;

    if takes_tp1(p) != 13 {
        panic!("Expected takes_tp1 to return 13");
    }

    if a.x != 13 {
        panic!("Expected a.x to be 13");
    }

    if takes_int(a.x) != 17 {
        panic!("Expected takes_int to return 17");
    }

    {
        let q = 0;
        let t1_size = std::mem::size_of::<S1>();
        let t1_size = q + t1_size;
        if t1_size == 0 {
            panic!("Expected size of S1 to be greater than 0");
        }
    }

    {
        let mut c = S2 { x: 30 };
        let mut r = &mut c;
        *r = S2 { x: 30 };

        if r.x != 30 {
            panic!("Expected r.x to be 30");
        }

        if takes_t2(c.clone()) != 35 {
            panic!("Expected takes_t2 to return 35");
        }
    }
}