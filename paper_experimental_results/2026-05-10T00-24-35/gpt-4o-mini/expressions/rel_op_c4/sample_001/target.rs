fn main() -> Result<(), i32> {
    let x = 10;
    let y = 20;

    let px: &i32 = &x;
    let py: &i32 = &y;

    let px_end: &i32 = &x + 1;
    let py_end: &i32 = &y + 1;

    if px != &x {
        return Err(1);
    }

    if !(px_end > px) {
        return Err(2);
    }

    if px_end != (&x + 1) {
        return Err(3);
    }

    if !(py_end > py) {
        return Err(4);
    }

    Ok(())
}

fn main() {
    std::process::exit(match main() {
        Ok(()) => 0,
        Err(code) => code,
    });
}