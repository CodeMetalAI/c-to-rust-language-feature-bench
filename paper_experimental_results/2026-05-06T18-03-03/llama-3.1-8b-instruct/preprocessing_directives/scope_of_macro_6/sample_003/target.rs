// scope_of_macro_6.rs

macro_rules! obj_like {
    () => (1 - 1);
}

macro_rules! func_like {
    ($a:expr) => ($a);
}

#[cfg(feature = "negative")]
macro_rules! obj_like {
    () => (0);
    (1e - 1) => (1e - 1);
}
#[cfg(feature = "negative")]
macro_rules! func_like {
    ($b:expr) => ($b);
    ($a:expr) => ($a);
}

fn main() {
    let x = obj_like!();
    let y = func_like!(3);

    if x != 0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}