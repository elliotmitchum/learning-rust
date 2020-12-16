fn main () {
    let a = 2;
    let b = 3;
    println!("{} + {} = {}", a, b, a + b);

    // Example of inferred types.
    let x = false;
    let y = x && true;
    println!("y is {}", y);

    // Example array usage.
    let arr: [i8; 3] =  [1, 4, 7];
    println!("{}", arr[0]);
}
