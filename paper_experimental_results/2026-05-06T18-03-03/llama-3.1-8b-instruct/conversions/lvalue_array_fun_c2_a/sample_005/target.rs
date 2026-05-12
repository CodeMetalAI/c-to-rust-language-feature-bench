fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx: i32 = 9;
    let mut ax: i32 = 11;

    let a: [i32; 3] = [1, 2, 3];

    let pa: &i32 = &a[0];
    if *pa!= 1 {
        panic!("pa[0]!= 1");
    }
    if *pa!= 2 {
        panic!("pa[1]!= 2");
    }
    if *pa!= 3 {
        panic!("pa[2]!= 3");
    }

    if std::any::type_id(&a)!= std::any::type_id(&[1, 2, 3]) {
        panic!("TYPE_ID(a)!= 99");
    }
    if std::any::type_id(&a[0])!= std::any::type_id(&1) {
        panic!("TYPE_ID(&a[0])!= 2");
    }

    if std::any::type_id(&cx)!= std::any::type_id(&9) {
        panic!("TYPE_ID(&cx)!= 3");
    }
    if std::any::type_id(cx)!= std::any::type_id(9) {
        panic!("TYPE_ID(+cx)!= 1");
    }
    if cx!= 9 {
        panic!("(+cx)!= 9");
    }

    if std::any::type_id(&ax)!= std::any::type_id(&11) {
        panic!("TYPE_ID(&ax)!= 4");
    }
    if std::any::type_id(&ax)!= std::any::type_id(&11) {
        panic!("TYPE_ID(&ax)!= 5");
    }
    if std::any::type_id(&ax)!= std::any::type_id(&11) {
        panic!("TYPE_ID(+ax)!= 1");
    }
    if ax!= 11 {
        panic!("(+ax)!= 11");
    }

    let fp: fn(i32) -> i32 = id;
    if std::any::type_id(id)!= std::any::type_id(|x| x + 1) {
        panic!("TYPE_ID(id)!= 6");
    }
    if fp(4)!= 5 {
        panic!("fp(4)!= 5");
    }
    if id(4)!= 5 {
        panic!("id(4)!= 5");
    }

    if std::mem::size_of::<[i32; 3]>()!= 3 * std::mem::size_of::<i32>() {
        panic!("sizeof a!= 3 * sizeof(int)");
    }
    if std::mem::align_of::<i32>() == std::mem::align_of::<i32>() {
        panic!("_Alignof(a[0])!= _Alignof(int)");
    }

    assert!(true);
}