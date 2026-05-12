fn main() {
    let mut x = 0;

    {
        goto end;
        x = 1;
    end:;
    }

    x += 1;
    let exit_code = if x == 1 { 0 } else { 1 };
    std::process::exit(exit_code);
}