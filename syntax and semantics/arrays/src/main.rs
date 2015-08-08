fn main() {
    let a = [0,1,2,3,4];
    let a1: [i32; 3] = [1,2,3];
    let mut m = [1,2,3];

    //slices
    let middle = &a[1..4];
    let complete = &a[..];
    
    //initialize each element of array with a value
    let b = [0;20];
    //or
    let b1: [i32; 20] = [0; 20];
    let length = b.len();

    println!("a has {} elements", a.len());
}
