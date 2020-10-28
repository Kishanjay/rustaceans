pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let x2: i64 = 3249283592853;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max f32: {}", std::f32::MAX);
    println!("Max f64: {}", std::f64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x,y,x2,is_active, is_greater, a1, face));

}