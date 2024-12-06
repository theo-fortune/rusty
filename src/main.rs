fn main() {
    println!("Hello, 🦀 from CARGO!");
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    let letter: char = 'a';
    println!("First letter of the alphabet is {}", letter);

    let numbers: [i32; 5]  = [1,2,3,4,5];
    println!("The Numbers {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Banana", "Pineapple"];
    println!("Fruits array 1st element: {}", fruits[0]);
    println!("Fruits array 2nd element: {}", fruits[1]);
    println!("Fruits array 3rd element: {}", fruits[2]);


}
