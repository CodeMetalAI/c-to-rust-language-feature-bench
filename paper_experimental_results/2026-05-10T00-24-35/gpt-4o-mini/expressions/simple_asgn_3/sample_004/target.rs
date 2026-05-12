fn main() -> std::process::ExitCode {
    let c = 'A';
    let mut p: Option<&char> = None;

    p = Some(&c);
    let p_value = p.unwrap_or(&'\0');

    if *p_value != 'A' {
        return std::process::ExitCode::from(1);
    }

    std::process::ExitCode::from(0)
}