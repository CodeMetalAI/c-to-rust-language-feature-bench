fn f(p: &dyn std::any::Any) {
    std::mem::forget(p);
}

fn f1(p: &dyn std::any::Any) {
    std::mem::forget(p);
}

fn f2(p: &dyn std::any::Any) {
    std::mem::forget(p);
}

fn f4(p: &dyn std::any::Any) {
    std::mem::forget(p);
}

fn f3(p: &dyn std::any::Any) {
    std::mem::forget(p);
}

fn main() {
    f(1);
    f(0);

    f2(1);
    f2(0);

    f3(1);
    f3(0);

    f(1);
    f(0);

    f4(1);
    f4(0);

    f1(1);
    f1(0);
}