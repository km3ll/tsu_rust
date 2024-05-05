use rand::Rng;

pub fn control_flow() {

    let random: i32 = rand::thread_rng().gen_range(1..=10);

    if random > 5 {
        println!("Random is bigger than 5");
    } else if random > 3 {
        println!("Random is bigger than 3");
    } else {
        println!("Random is smaller of equal to 3");
    }

}