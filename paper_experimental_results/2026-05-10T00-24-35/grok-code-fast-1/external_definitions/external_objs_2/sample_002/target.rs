use std::cell::RefCell;

thread_local! {
    static I: RefCell<Vec<i32>> = RefCell::new(vec![0]);
}

fn main() {
    {
        let i = I.borrow();
        if i[0] != 0 {
            std::process::exit(1);
        }
    }
    {
        let mut i = I.borrow_mut();
        i[0] = 7;
        if i[0] != 7 {
            std::process::exit(2);
        }
    }
    std::process::exit(0);
}