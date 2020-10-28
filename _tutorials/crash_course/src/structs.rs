// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // this method takes over the ownership of the Person, therefore it's
    // previous reference will be invalidated
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut t = Color2(255, 0, 0);
    t.0 = 200;
    println!("Color: {} {} {}", t.0, t.1, t.2);


    let mut p = Person::new("Kishan", "Nirghin");
    println!("Person {}", p.full_name());
    println!("Person {} {}", p.first_name, p.last_name);

    let mut p2 = Person::new("Kishan", "Nirghin");
    p2.set_last_name("N.");
    println!("Person {}", p2.full_name());
    println!("Person Tuple {:?}", p2.to_tuple());
    
}