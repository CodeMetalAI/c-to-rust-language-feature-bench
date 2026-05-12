fn main() -> std::process::ExitCode {
    let s = "12345";
    let mut acc = 0;
    let mut chars = s.chars();
    
    while let Some(_) = chars.next() {
        acc += 1;
    }
    
    if acc == 5 {
        std::process::ExitCode::SUCCESS
    } else {
        std::process::ExitCode::FAILURE
    }
}