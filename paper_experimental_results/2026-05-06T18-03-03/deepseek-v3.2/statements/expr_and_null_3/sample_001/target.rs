fn main() {
    let mut x = 0;

    {
        goto_end!();
        x = 1;
        end: {}
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}

macro_rules! goto_end {
    () => {
        return ();
    };
}