// std::rc and std::cell::RefCell are used to mimic the behavior of pointers
use std::rc::Rc;
use std::cell::RefCell;

// Define a struct s1
struct S1 {
    x: i32,
}

// Define a struct s2
struct S2 {
    x: i32,
}

// Define a function takes_t1 that takes a S1 and returns an i32
fn takes_t1(v: S1) -> i32 {
    v.x + 1
}

// Define a function takes_s1 that takes a S1 and returns an i32
fn takes_s1(v: S1) -> i32 {
    v.x + 2
}

// Define a function takes_tp1 that takes a Rc<S1> and returns an i32
fn takes_tp1(p: Rc<RefCell<S1>>) -> i32 {
    p.borrow_mut().x += 3;
    p.borrow().x
}

// Define a function takes_int that takes an i32 and returns an i32
fn takes_int(v: i32) -> i32 {
    v + 4
}

// Define a function takes_t2 that takes a S2 and returns an i32
fn takes_t2(v: S2) -> i32 {
    v.x + 5
}

fn main() {
    // Create a S1 and assign it the value of 10
    let mut a = S1 { x: 10 };

    // Create a RC S1 and assign it the value of 20
    let b = Rc::new(RefCell::new(S1 { x: 20 }));

    // Check if takes_t1(a) equals 11
    if takes_t1(a.clone()) != 11 {
        panic!("Expected 11 from takes_t1");
    }

    // Check if takes_s1(a) equals 12
    if takes_s1(a) != 12 {
        panic!("Expected 12 from takes_s1");
    }

    // Check if takes_t1(b) equals 21
    if takes_t1(b.borrow().clone()) != 21 {
        panic!("Expected 21 from takes_t1");
    }

    // Check if takes_s1(b) equals 22
    if takes_s1(b.borrow().clone()) != 22 {
        panic!("Expected 22 from takes_s1");
    }

    // Create a Rc S1 and assign it the value of a
    let p = Rc::new(RefCell::new(a));

    // Check if takes_tp1(p) equals 13
    if takes_tp1(p.clone()) != 13 {
        panic!("Expected 13 from takes_tp1");
    }

    // Check if a.x equals 13
    if a.x != 13 {
        panic!("Expected a.x to equal 13");
    }

    // Check if takes_int(a.x) equals 17
    if takes_int(a.x) != 17 {
        panic!("Expected 17 from takes_int");
    }

    // Check if the size of S1 is not zero
    let s1_size = std::mem::size_of::<S1>();
    let s2_size = std::mem::size_of::<S2>();
    if s1_size + s2_size != 0 {
        panic!("Expected size to be 0");
    }

    // Create a S2 and assign it the value of 30
    let mut c = S2 { x: 30 };

    // Create a Rc S2 and assign it the value of c
    let r = Rc::new(RefCell::new(c));

    // Check if r.x equals 30
    if r.borrow().x != 30 {
        panic!("Expected r.x to equal 30");
    }

    // Check if takes_t2(c) equals 35
    if takes_t2(c) != 35 {
        panic!("Expected 35 from takes_t2");
    }
}