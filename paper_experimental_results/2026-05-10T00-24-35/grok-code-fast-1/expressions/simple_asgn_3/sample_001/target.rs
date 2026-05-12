use std::cell::Cell;
use std::rc::Rc;

fn main() {
    let c: Rc<Cell<char>> = Rc::new(Cell::new('A'));
    let mut p: Option<Rc<Cell<char>>> = None;
    let cpp: &mut Option<Rc<Cell<char>>> = &mut p;
    *cpp = Some(c.clone());
    if let Some(ref p_ref) = p {
        p_ref.set('\0');
    }
    let c_val = c.get();
    if c_val != 'A' {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}