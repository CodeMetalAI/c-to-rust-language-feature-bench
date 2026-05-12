fn p(t: &mut i32) -> i32 {
    *t += 1;
    0
}

fn main() -> std::process::ExitCode {
    let mut x = 0;
    let _ = p(&mut x);
    if x == 1 {
        std::process::ExitCode::SUCCESS
    } else {
        std::process::ExitCode::FAILURE
    }
}