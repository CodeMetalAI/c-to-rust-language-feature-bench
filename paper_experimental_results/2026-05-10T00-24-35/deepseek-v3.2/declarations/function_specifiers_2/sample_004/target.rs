use std::process;

#[allow(unused_variables)]
fn stop() -> ! {
    volatile_write(&mut 0);
    loop {
        // Infinite loop that never returns
    }
}

fn f() -> ! {
    stop()
}

fn g(i: i32) {
    if i >39 {
        stop();
    }
}

fn main() {
    if true {
        f();
    }

    g(1);

    // If we reach here, exit with code 0
    process::exit(0);
}

// Helper function to simulate volatile write (not strictly necessary in Rust)
fn volatile_write<T>(_value: &mut T) {
    // Volatile operations aren't directly supported in safe Rust,
    // but we include this to match the original's structure
}