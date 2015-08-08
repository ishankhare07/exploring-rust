fn main() {
    //let (x,y,z) = (1,2,3);
    (0,);       //single element tuple
    (0);        //0 in parentheses

    let tuple = (10,20,30);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("{},{},{}", x,y,z);
}
