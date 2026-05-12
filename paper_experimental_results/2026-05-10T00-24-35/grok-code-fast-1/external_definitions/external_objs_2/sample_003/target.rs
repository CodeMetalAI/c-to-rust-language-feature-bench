use std::cell::RefCell;
use std::process;

thread_local! {
    static I: RefCell<[i32; 1]> = RefCell::new([0]);
}

fn main() {
    let exit_code = I.with(|i| {
        let mut i = i.borrow_mut();
        if i[0] != 0 {
            1
        } else {
            i[0] = 7;
            if i[0] != 7 {
                2
            } else {
                0
            }
        }
    });
    process::exit(exit_code);
}