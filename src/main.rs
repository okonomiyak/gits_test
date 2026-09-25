fn main() {
    for i in 0..100{
        match (i%3,i%5) {
            (0,0) => println!("fizzbuzz"),
            (0,_) => println!("buzz"),
            (_,0) => println!("fizz"),
            (_,_) => println!("{i}")
        }
    }
}
