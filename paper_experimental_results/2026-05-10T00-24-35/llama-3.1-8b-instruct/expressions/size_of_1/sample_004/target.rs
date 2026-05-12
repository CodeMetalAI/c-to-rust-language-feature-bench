fn main() {
    let dp = Box::new(3.14);
    if *dp!= 3.14 {
        return 2;
    }
    drop(dp);
    return 0;
}