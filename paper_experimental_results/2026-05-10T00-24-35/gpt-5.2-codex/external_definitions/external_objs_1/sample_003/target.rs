use std::cell::Cell;
use std::rc::Rc;

thread_local! {
    static I1: Rc<Cell<i32>> = Rc::new(Cell::new(1));
    static I2: Rc<Cell<i32>> = Rc::new(Cell::new(2));
    static I3: Rc<Cell<i32>> = Rc::new(Cell::new(3));
    static I4: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    static I5: Rc<Cell<i32>> = Rc::new(Cell::new(0));

    static PI1: Rc<Cell<i32>> = I1.with(|v| v.clone());
    static PI2: Rc<Cell<i32>> = I2.with(|v| v.clone());
    static PI4: Rc<Cell<i32>> = I4.with(|v| v.clone());
    static PI5: Rc<Cell<i32>> = I5.with(|v| v.clone());
}

fn i1_rc() -> Rc<Cell<i32>> { I1.with(|v| v.clone()) }
fn i2_rc() -> Rc<Cell<i32>> { I2.with(|v| v.clone()) }
fn i3_rc() -> Rc<Cell<i32>> { I3.with(|v| v.clone()) }
fn i4_rc() -> Rc<Cell<i32>> { I4.with(|v| v.clone()) }
fn i5_rc() -> Rc<Cell<i32>> { I5.with(|v| v.clone()) }

fn p_i1_rc() -> Rc<Cell<i32>> { PI1.with(|v| v.clone()) }
fn p_i2_rc() -> Rc<Cell<i32>> { PI2.with(|v| v.clone()) }
fn p_i4_rc() -> Rc<Cell<i32>> { PI4.with(|v| v.clone()) }
fn p_i5_rc() -> Rc<Cell<i32>> { PI5.with(|v| v.clone()) }

fn get_i1() -> i32 { I1.with(|v| v.get()) }
fn get_i2() -> i32 { I2.with(|v| v.get()) }
fn get_i3() -> i32 { I3.with(|v| v.get()) }
fn get_i4() -> i32 { I4.with(|v| v.get()) }
fn get_i5() -> i32 { I5.with(|v| v.get()) }

fn set_i1(val: i32) { I1.with(|v| v.set(val)); }
fn set_i2(val: i32) { I2.with(|v| v.set(val)); }
fn set_i4(val: i32) { I4.with(|v| v.set(val)); }
fn set_i5(val: i32) { I5.with(|v| v.set(val)); }

fn get_p_i1() -> i32 { PI1.with(|v| v.get()) }
fn get_p_i2() -> i32 { PI2.with(|v| v.get()) }
fn get_p_i4() -> i32 { PI4.with(|v| v.get()) }
fn get_p_i5() -> i32 { PI5.with(|v| v.get()) }

fn main() {
    if get_i1() != 1 { std::process::exit(1); }
    if get_i2() != 2 { std::process::exit(2); }
    if get_i3() != 3 { std::process::exit(3); }

    if get_i4() != 0 { std::process::exit(4); }
    if get_i5() != 0 { std::process::exit(5); }

    let i1 = i1_rc();
    let p_i1 = p_i1_rc();
    if !Rc::ptr_eq(&i1, &p_i1) { std::process::exit(6); }

    let i2 = i2_rc();
    let p_i2 = p_i2_rc();
    if !Rc::ptr_eq(&i2, &p_i2) { std::process::exit(7); }

    let i4 = i4_rc();
    let p_i4 = p_i4_rc();
    if !Rc::ptr_eq(&i4, &p_i4) { std::process::exit(8); }

    let i5 = i5_rc();
    let p_i5 = p_i5_rc();
    if !Rc::ptr_eq(&i5, &p_i5) { std::process::exit(9); }

    set_i1(10);
    set_i2(20);
    set_i4(30);
    set_i5(40);

    if get_p_i1() != 10 { std::process::exit(10); }
    if get_p_i2() != 20 { std::process::exit(11); }
    if get_p_i4() != 30 { std::process::exit(12); }
    if get_p_i5() != 40 { std::process::exit(13); }

    std::process::exit(0);
}