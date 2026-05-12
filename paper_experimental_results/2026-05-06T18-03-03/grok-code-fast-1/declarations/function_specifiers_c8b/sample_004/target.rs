use std::sync::OnceLock;

static G: OnceLock<i32> = OnceLock::new();

fn die_if(x: i32) -> ! {
    let g_val = G.get().unwrap();
    if x == 7 && *g_val == 9 {
        std::process::exit(0);
    } else {
        std::process::exit(2);
    }
}

fn main() {
    G.set(9).unwrap();
    let p: fn(i32) = die_if;
    p(7);
    // In C, return 3; but unreachable, in Rust main returns () which exits 0, but since we exit above, fine.
}