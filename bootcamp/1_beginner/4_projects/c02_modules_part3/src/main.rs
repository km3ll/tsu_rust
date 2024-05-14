#![allow(unused)]

use c02_modules_part3::{Credentials};

fn main() {

    let creds = Credentials {
        username: String::from("letsgetrusty"),
        password: String::from("password1234"),
    };

    c02_modules_part3::authenticate(creds);

}