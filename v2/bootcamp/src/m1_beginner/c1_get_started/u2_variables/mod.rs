pub fn variables() {
    println!("----------");
    println!("Variables");

    println!("creation");
    let a1: i16 = 5;
    let a2: f32 = 5.0;
    println!("a1: {a1}");
    println!("a2: {a2}");

    println!("mutability");
    let mut m1: i16 = 4;
    println!("m1: {m1}");
    m1 = 6;
    println!("m1: {m1}");

    println!("shadowing");
    let s1: i32 = 10;
    let s1: i32 = 20;
    println!("s1: {s1}");

    println!("scope");
    let d1: i16 = 40;
    {
        // This d1 lives within the scope of brackets {}
        let d1: i16 = 30;
        println!("inner d1: {d1}");
    }
    println!("outer d1: {d1}");

}