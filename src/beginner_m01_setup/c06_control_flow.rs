use rand::Rng;

pub fn control_flow() {

    let random: i32 = rand::thread_rng().gen_range(-10..=10);
    println!("Random value is {}", random);

    // if/else
    if random > 5 {
        println!("Bigger than 5");
    } else if random > 3 {
        println!("Bigger than 3");
    } else {
        println!("Smaller of equal to 3");
    }

    // if/else expressions can be used in let statements.
    let b: i32 = if random > 0 { 1 } else { -1 }; 

    // loops
    /*
    loop {
        println!("Loop forever");
    }
    */
    loop {
        println!("Hello from loop");
        break;
    }

    // labeling
    'outer: loop {
        println!("Hello from outer");
        'inner: loop {
            println!("Hello from inner");
            break 'outer;
        }
        println!("This will never be printed");
    }

    // returning value
    let x: i32 = loop {
        println!("Hello from returning loop");
        break 10;
    };
    println!("Value returned from loop was: {}", x);

    // while
    let mut a: i32 = 1;
    while a < 5 {
        println!("Value of a: {}", a);
        a += 1;
    }

    // for-loop
    let names: [&str; 4] = ["John", "Anne", "Mike", "Envera"];
    for name in names {
        println!("For-loop name: {}", name);
    }

}