fn main() {
    for x in 1..10 {
        println!("{}",x);
    }

    for (i,j) in (5..10).enumerate() {
        println!("i = {} and j = {}",i,j);
    }
}
