fn f<T: Copy>(p: Option<&T>) {}
fn f1<T>(p: Option<&T>) {}
fn f2(p: Option<&volatile i32>) {}
fn f4(p: Option<&i32>) {}
fn f3(p: Option<&volatile i32>) {}

fn main() {
    f(Some(1).then(|| std::ptr::null::<i32>));
    f(Some(std::ptr::null::<i32>()).then(|| 1));

    f2(Some(1).then(|| std::ptr::null::<i32>));
    f2(Some(std::ptr::null::<i32>()).then(|| 1));

    f3(Some(std::ptr::null::<i32>()).then(|| 1));
    f3(Some(1).then(|| std::ptr::null::<i32>));

    f(Some(std::ptr::null::<i32>()).then(|| "Hello, world!".as_ptr()));
    f(Some("Hello, world!".as_ptr()).then(|| std::ptr::null::<i32>));

    f4(Some(1).then(|| std::ptr::null::<i32>));
    f4(Some(std::ptr::null::<i32>()).then(|| 1));

    f1(Some(std::ptr::null::<i32>()).then(|| 1));
    f1(Some(1).then(|| std::ptr::null::<i32>));

    std::process::exit(0);
}