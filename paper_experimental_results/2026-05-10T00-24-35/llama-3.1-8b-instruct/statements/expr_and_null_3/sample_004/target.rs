fn main() {
    let mut x = 0;

    {
        // The scope of the block starts here, so x is dropped if we jump here
        break;  // 'break' is used to immediately exit the loop or block
        x = 1;
    }

    // x is not dropped here because we fall through the block
    x += 1;
    assert!(x == 1, "Expected x to be 1");
    std::process::exit(0);  // equivalent to returning 0
}