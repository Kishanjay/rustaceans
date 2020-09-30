pub fn run() {
    // Print to console
    println!("Hello from the prin.rs file");

    // Basic Formatting
    println!("{} is {} + {}", "5", "2", "3");

    // Positional Arguments
    println!("Hello {0}, and not Hello {0}. Hi {1}", "World", "Kishan");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "Kishan", activity = "Squash");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}