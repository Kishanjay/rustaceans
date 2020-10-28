pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign value
    numbers[3] = 420;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("Singe Value: {}", numbers[0]);

    // Get Vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies: {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] =  &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop through vector values with indexes
    for (i, x) in numbers.iter().enumerate() {
        println!("Number: {}:{}", i, x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x = *x*2;
    }

    // Concat vectors
    let mut a: Vec<i32> = vec![1,2,3];
    let mut b: Vec<i32> = vec![4,5,6];
    a.append(&mut b);
    println!("{:?}", a);

    println!("Numbers Vec: {:?}", numbers);
}