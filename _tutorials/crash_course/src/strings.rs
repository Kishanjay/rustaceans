pub fn run() {
    // Immutable
    let hello = "Hello";

    // Growable, heap allocated
    let mut hello2 = String::from("Hello");

    // Get length
    println!("Length: {}", hello2.len());

    // Push char
    hello2.push('A');

    // Push string
    hello2.push_str("n example");

    // Capacity in bytes
    println!("Capacity: {}", hello2.capacity());

    // Contains
    println!("Contains 'World' {}", hello2.contains("World"));

    // Replace
    println!("Replace: {}", hello2.replace("Hello", "Hello "));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    
    println!("{}", hello2);
    println!("{}", hello);

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    
    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{:?}", s);
}