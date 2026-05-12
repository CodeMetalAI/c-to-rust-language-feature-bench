const TABSIZE: usize = 100;

fn main() {
    let a = [0; TABSIZE];
    if a.len()!= 100 {
        std::process::exit(1);
    }
    let tab_size = 7;
    if tab_size!= 7 {
        std::process::exit(2);
    }
    std::process::exit(0);
}