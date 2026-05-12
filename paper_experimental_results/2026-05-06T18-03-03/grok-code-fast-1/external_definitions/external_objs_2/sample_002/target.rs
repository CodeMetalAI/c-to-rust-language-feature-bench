use std::cell::RefCell;

thread_local! {
    static I: RefCell<[i32; 1]> = RefCell::new([0]);
}

fn main() {
    I.with(|i| {
        let mut i = i.borrow_mut();
        if i[0] != 0 {
            std::process::exit(1);
        }
        i[0] = 7;
        if i[0] != 7 {
            std::process::exit(2);
        }
        std::process::exit(0);
    });
}