fn variable_mutability() {
    let x = "hello world";

    if true {
        let x = "new variable";
        println!("{}", x);
    }

    let x = "overwritten";
    println!("{}", x);
}

fn integer_types() {
    println!("{}", i8::MAX);
    println!("{}", u8::MAX);
}

fn expressions() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("Scopes can return a value: {}", y);
}

fn control_flow() {
    let mut i = 0;
    let result = loop {
        i += 1;

        if i == 5 {
            break false;
        }
        if i == 6 {
            break true;
        }
    };

    println!("Loops can return a value {}", result);

    for i in (1..4).rev() {
        println!("Countdown: {}", i);
    }
}



fn main() {
    variable_mutability();
    integer_types();
    expressions();
    control_flow();
}
