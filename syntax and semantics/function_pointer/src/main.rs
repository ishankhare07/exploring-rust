fn foo(x: i32) -> i32 { x }

fn main() {
    let x: fn(i32) -> i32 = foo;
    println!("{}", x(10));
}
