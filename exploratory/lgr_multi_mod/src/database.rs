#[derive(Debug)]
pub enum Status {
    Connected,
    Interrupted,
}

pub fn connect_to_database() -> Status {
    println!(" > func: connect_to_database()");
    Status::Connected
}

pub fn get_user() {
    println!(" > func: get_user()");
}
