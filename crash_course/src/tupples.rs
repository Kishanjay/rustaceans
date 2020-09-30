pub fn run () {
    // Max 12 items inside a tupple
    let person: (&str, &str, i8) = ("Kishan", "Nirghin", 26);

    println!("{} {} is {}", person.0, person.1, person.2);
}