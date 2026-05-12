fn main() -> std::process::ExitCode {
    let s = "12345";
    let mut acc = 0;
    let mut chars = s.chars();
    
    loop {
        match chars.next() {
            Some(_) => acc += 1,
            None => break,
        }
    }
    
    if acc == 5 {
        std::process::ExitCode::SUCCESS
    } else {
        std::process::ExitCode::FAILURE
    }
}