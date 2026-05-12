use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    let result = {
        let x = AtomicI32::new(41);

        let mut old = x.fetch_add(1, Ordering::SeqCst);
        if old != 41 {
            1
        } else if x.load(Ordering::SeqCst) != 42 {
            2
        } else {
            old = x.fetch_add(1, Ordering::SeqCst);
            if old != 42 {
                3
            } else if x.load(Ordering::SeqCst) != 43 {
                4
            } else {
                x.store(100, Ordering::SeqCst);
                let y = x.fetch_add(1, Ordering::SeqCst) + 0;
                if y != 100 {
                    5
                } else if x.load(Ordering::SeqCst) != 101 {
                    6
                } else {
                    0
                }
            }
        }
    };

    exit(result);
}