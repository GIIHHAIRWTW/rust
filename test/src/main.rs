fn main() {
    for i in 1..10 {
        for j in 1..i+1 {
            print!("{}*{}={}\t", j, i, i*j)
        }
        println!()
    }
}

#[test]
fn test() {
    println!("Hello, Test!")
}
