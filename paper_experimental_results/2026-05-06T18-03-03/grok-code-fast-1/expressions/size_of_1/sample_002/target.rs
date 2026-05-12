use std::panic;

fn main() {
    let result = panic::catch_unwind(|| Box::new(0.0f64));
    let mut dp = match result {
        Ok(boxed) => boxed,
        Err(_) => return,
    };
    *dp = 3.14;
    if *dp != 3.14 {
        return;
    }
}