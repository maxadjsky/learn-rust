
fn main() {
    let x = 5;
    println!("The value of x is {}",x );
    // the following codes will throw an error `re-assignment of immutable variable`
    // x = 6;
    // println!("The value of x is {}",x );
    let mut y = 6;
    println!("The value of y is {}", y);

    y = 10;
    println!("The value of y is {}",y );
}
