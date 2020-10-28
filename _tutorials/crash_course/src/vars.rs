pub fn run() {
    let name = "Kishan";
    let mut age = 23;
    println!("My name is {} and I am {} years old", name, age);
    age = 26;
    println!("My name is {} and I am {} years old", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Kishan", 26);
    println!("{} is {}", my_name, my_age);
}