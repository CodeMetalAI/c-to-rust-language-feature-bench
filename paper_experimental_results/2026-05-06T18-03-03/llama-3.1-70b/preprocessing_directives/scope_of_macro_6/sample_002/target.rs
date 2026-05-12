fn main() {
    let obj_like = 0;
    let func_like = |a| a;

    let x = obj_like;
    let y = func_like(3);

    if x!= 0 {
        std::process::exit(1);
    }
    if y!= 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}