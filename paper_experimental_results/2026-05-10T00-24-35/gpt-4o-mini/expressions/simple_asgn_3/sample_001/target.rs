fn main() -> Result<(), std::io::Error> {
    let c: char = 'A';
    let mut p: Option<&char> = None;

    p = Some(&c);
    if c != 'A' {
        std::process::exit(1);
    }

    Ok(())
}