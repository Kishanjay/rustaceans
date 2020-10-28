pub fn run() {
    let age: u8 = 18;
    let checked_id: bool = false;
    let knows_person_of_age = true;

    // If/Else
    if age >= 21 && checked_id || knows_person_of_age {
        println!("Bartender: What would like to drink");
    } else if age < 21 && checked_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I need an ID please")
    }

    // Shorthand If
    let is_of_age = if age >= 21 {true} else { false };
    println!("Is of age: {}", is_of_age);
}