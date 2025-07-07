fn main() {
    println!("Hello, world!");

    let name = "John";
    let age:u8 = 25;  // Inmutable
    let mut age_mut = 25;  // Mut = mutable
    age_mut += 1;

    println!("My name is {} and I am {} years old.", name, age);
    println!("My name is {} and I am {} years old.", name, age_mut);
}
