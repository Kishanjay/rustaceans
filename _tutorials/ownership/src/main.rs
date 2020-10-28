fn main() {
    println!("Hello, world!");

    // heap allocated data is stored as a pointer, for the rust language to know
    // when this allocated memory can be used it will be dropped whenever the
    // scope ends.
    let x = String::from("This will be heap allocated");

    // since this will make a copy of the pointer x, there are 2 instances 
    // pointing to the same value. Dropping the value twice is insecure 
    // therefore the value x is invalidated; and ownership is transfered to y.
    let y = x;

    // println!("{}", x); // THIS IS INVALID

    // the ownership of y is transfered to this function therefore y is invalid
    // after this function
    takeOwnership(y);


    // the ownership will be passed to the function who will pass it back
    let z = String::from("Lets pass around this ownership");
    let z = boomerrangOwnership(z);

    // pass a pointer to the local variable (which doesn't transfer ownership)
    referenceOnly(&z);

    // pass a mutable reference..
    mutableReference(&mut z);
}


fn takeOwnership(s: String){ 
    println!("{}", s);
}

fn boomerrangOwnership(s: String) -> String {
    s
}

fn referenceOnly(s: &String) {
    println!("{}", s);
    println!("{}", *s);
    println!("{}", s.len());
    println!("{}", (*s).len());
}

fn mutableReference(s: &mut String) {
    s.push_str("Wow");
}