#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}

#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

pub mod front_of_house {
    pub fn deliver_order() {
        println!(" > order delivered");
    }
}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::deliver_order();
    }
    fn cook_order() {
        println!(" > order cooked");
    }
}
