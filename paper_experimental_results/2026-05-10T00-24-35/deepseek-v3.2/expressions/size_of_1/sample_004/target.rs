fn main() {
    let dp = Box::<f64>::new(3.14);
    
    if *dp != 3.14 {
        std::process::exit(2);
    }
    
    std::process::exit(0);
}