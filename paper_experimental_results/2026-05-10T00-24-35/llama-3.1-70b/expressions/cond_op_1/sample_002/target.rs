fn f(_p: &dyn std::any::Any) {}
fn f1(_p: &dyn std::any::Any) {}
fn f2(_p: &mut i32) {}
fn f4(_p: &i32) {}
fn f3(_p: &i32) {}

fn main() {
    f(&std::ptr::null::<()>() as *const () as &dyn std::any::Any);
    f(&1 as &i32 as &dyn std::any::Any);

    f2(&mut 0);
    f2(&mut 0);

    f3(&1);
    f3(&1);

    f(&std::ptr::null::<()>() as *const () as &dyn std::any::Any);
    f(&std::ptr::null::<()>() as *const () as &dyn std::any::Any);

    f4(&1);
    f4(&1);

    f1(&std::ptr::null::<()>() as *const () as &dyn std::any::Any);
    f1(&std::ptr::null::<()>() as *const () as &dyn std::any::Any);
}