fn main() {
    let mut x = 0;

    {
        // This block will be skipped due to the goto statement
        // However, the variable x is still accessible here
        // But the goto statement will jump to the end label
        // So, the code inside this block will be skipped
        // and the code after this block will be executed
        let _ = x; // This line is just to make the compiler happy
        x = 1;
    }

    x += 1;
    assert_eq!(x, 1);
    std::process::exit(if x == 1 { 0 } else { 1 });
}