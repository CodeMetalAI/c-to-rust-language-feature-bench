fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn check_name(got: &str, expect: &str) -> bool {
    streq(got, expect)
}

fn f() -> i32 {
    let function_name = "f";
    if !check_name(function_name, "f") {
        return -1;
    }
    0
}

fn g() -> i32 {
    let function_name = "g";
    if !check_name(function_name, "g") {
        return -2;
    }
    0
}

fn main() {
    let function_name = "main";
    if !check_name(function_name, "main") {
        std::process::exit(1);
    }
    let result = f() + g();
    println!("{}", result);
}