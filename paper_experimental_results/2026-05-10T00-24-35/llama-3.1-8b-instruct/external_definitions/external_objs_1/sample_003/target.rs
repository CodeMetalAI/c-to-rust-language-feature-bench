fn main() {
    let i1: i32 = 1;
    let i2: i32 = 2;
    let i3: i32 = 3;
    let i4: i32 = 0;
    let i5: i32 = 0;

    let _i1: i32 = i1;
    let _i2: i32 = i2;
    let _i3: i32 = i3;
    let _i4: i32 = i4;
    let _i5: i32 = i5;

    let i1: &i32 = &i1;
    let i2: &i32 = &i2;
    let i3: &i32 = &i3;
    let i4: &i32 = &i4;
    let i5: &i32 = &i5;

    let p_i1: &i32 = &i1;
    let p_i2: &i32 = &i2;
    let p_i4: &i32 = &i4;
    let p_i5: &i32 = &i5;

    if i1!= 1 {
        panic!("i1!= 1");
    }
    if i2!= 2 {
        panic!("i2!= 2");
    }
    if i3!= 3 {
        panic!("i3!= 3");
    }

    if i4!= 0 {
        panic!("i4!= 0");
    }
    if i5!= 0 {
        panic!("i5!= 0");
    }

    if p_i1!= &i1 {
        panic!("p_i1!= &i1");
    }
    if p_i2!= &i2 {
        panic!("p_i2!= &i2");
    }
    if p_i4!= &i4 {
        panic!("p_i4!= &i4");
    }
    if p_i5!= &i5 {
        panic!("p_i5!= &i5");
    }

    *i1 = 10;
    *i2 = 20;
    *i4 = 30;
    *i5 = 40;

    if *p_i1!= 10 {
        panic!("*p_i1!= 10");
    }
    if *p_i2!= 20 {
        panic!("*p_i2!= 20");
    }
    if *p_i4!= 30 {
        panic!("*p_i4!= 30");
    }
    if *p_i5!= 40 {
        panic!("*p_i5!= 40");
    }

    println!("All tests passed.");
}