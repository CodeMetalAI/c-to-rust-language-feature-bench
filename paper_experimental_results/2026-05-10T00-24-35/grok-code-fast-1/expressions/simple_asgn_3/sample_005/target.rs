use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let c = Rc::new(RefCell::new('A'));
    let mut p = None::<Rc<RefCell<char>>>;
    let cpp = &mut p;
    *cpp = Some(Rc::clone(&c));
    if let Some(ref cell) = *cpp {
        *cell.borrow_mut() = '\0';
    }
    let c_val = *c.borrow();
    if c_val != 'A' {
        std::process::exit(1);
    }
    std::process::exit(0);
}