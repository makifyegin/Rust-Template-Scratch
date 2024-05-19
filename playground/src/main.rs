fn main() {
    println!("Hello, world!");
    //Can you use sum function inside the lib.rs
    //to add two numbers?
    let a = 10;
    let b = 20;
    let result = playground::sum(a, b);
    println!("The sum of {} and {} is {}", a, b, result);

}
