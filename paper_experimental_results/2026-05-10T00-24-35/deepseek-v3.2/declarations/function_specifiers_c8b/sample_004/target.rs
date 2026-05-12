use std::process;

#[derive(Clone, Copy)]
struct DieIf;

impl std::ops::FnOnce<(i32,)> for DieIf {
    type Output = ();

    extern "rust-call" fn call_once(self, args: (i32,)) -> () {
        let (x,) = args;
        if x == 7 && crate::G == 9 {
            process::exit(0);
        } else {
            process::exit(2);
        }
    }
}

impl std::ops::FnMut<(i32,)> for DieIf {
    extern "rust-call" fn call_mut(&mut self, args: (i32,)) -> () {
        self.call_once(args)
    }
}

impl std::ops::Fn<(i32,)> for DieIf {
    extern "rust-call" fn call(&self, args: (i32,)) -> () {
        self.clone().call_once(args)
    }
}

static mut G: i32 = 0;

fn main() {
    let p: &dyn Fn(i32) -> () = &DieIf;
    unsafe {
        G = 9;
    }
    p(7);
    // This line is unreachable because process::exit terminates the program.
    process::exit(3);
}