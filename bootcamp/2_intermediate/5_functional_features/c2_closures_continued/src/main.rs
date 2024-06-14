
// In order to use Closures we need to use Generics and Trait Bounds
struct CredentialsV1 {
    username: String, 
    password: String,
}

struct CredentialsV2<T> where T: Fn(&str, &str) -> bool {
    username: String, 
    password: String,
    validator: T
}

impl<T> CredentialsV2<T> where T: Fn(&str, &str) -> bool {
    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

fn main() {

    println!("Hello, world!");

    let creds_v1 = CredentialsV1 {
        username: "".to_owned(),
        password: "".to_owned(),
    };

    // Closures are anonymous functions which you can store in variables or pass
    // as arguments to other functions. Closures can be stored and passed around.
    let validator_v2 = |username: &str, password: &str| -> bool {
        !username.is_empty() && !password.is_empty()    
    };

    let validator_v3 = |username: &str, password: &str| -> bool {
        !username.is_empty() &&
        !password.is_empty() &&
        password.len() < 8 &&
        password.contains(['!', '@', '#', '$', '%', '&', '*'])
    };

    // Closures can capture variables in the scope in which they are defined

    // The three closure functions are:
    // Fn     - Immutably borrow variables in environment
    // FnMut  - Mutably borrow variables in environment. Can change environment.
    // FnOnce - Take ownership of variables in environment. Can only be called once.

    // Every closure implements the FnOnce trait because every closure can be called
    // at least once.

    // We can enforce a closure to take ownership of a variable by using the 'move'
    // keyword. It can be used right before the closure definition:
    // let validator_v4 = move |username: &str, password: &str| -> bool {...}
    // One use for the move keyword is passing a closure to a newly created thread.

    let weak_password = "password123!".to_owned();
    let validator_v4 = |username: &str, password: &str| -> bool {
        !username.is_empty() &&
        !password.is_empty() &&
        password.len() < 8 &&
        password.contains(['!', '@', '#', '$', '%', '&', '*']) &&
        password != weak_password
    };

    let creds_v2 = CredentialsV2 {
        username: "john".to_owned(),
        password: "1234".to_owned(),
        validator: validator_v2
    };

    let creds_v3 = CredentialsV2 {
        username: "john".to_owned(),
        password: "".to_owned(),
        validator: validator_v3
    };
    let weak_password = "password123!".to_owned();
    let validator_v4 = |username: &str, password: &str| -> bool {
        !username.is_empty() &&
        !password.is_empty() &&
        password.len() < 8 &&
        password.contains(['!', '@', '#', '$', '%', '&', '*']) &&
        password != weak_password
    };

    let creds_v2 = CredentialsV2 {
        username: "john".to_owned(),
        password: "1234".to_owned(),
        validator: validator_v2
    };

    let creds_v3 = CredentialsV2 {
        username: "john".to_owned(),
        password: "".to_owned(),
        validator: validator_v3
    };

    let creds_v4 = CredentialsV2 {
        username: "john".to_owned(),
        password: "password123!".to_owned(),
        validator: validator_v4
    };

    let default_creds_v0 = get_default_creds(validator_v3);

    // Our code compile because Box implements Fn, FnMut and FnOnce
    // This means you can use a Box closure trait object where a generic implementation
    // a closure trait is expected
    let password_validator_v1 = get_password_validator_v1(8);
    let default_creds_v1 = get_default_creds(password_validator_v1);
    let password_validator_v2 = get_password_validator_v2(8, true);
    let default_creds_v2 = get_default_creds(password_validator_v2);

    println!("validate v1.1: {}", validate_credentials_v1(&creds_v1.username, &creds_v1.password));
    println!("validate v1.2: {}", validator_v2(&creds_v1.username, &creds_v1.password));
    println!("validate v2: {}", creds_v2.is_valid());
    println!("validate v3: {}", creds_v3.is_valid());
    println!("validate v4: {}", creds_v4.is_valid());

    println!("validate v5: {}", default_creds_v0.is_valid());
    println!("validate v6: {}", default_creds_v1.is_valid());
    println!("validate v7 {}", default_creds_v2.is_valid());
    
}


fn validate_credentials_v1(username: &str, password: &str) -> bool { 
    !username.is_empty() && !password.is_empty()
}

// Function accepting a closure as an argument
fn get_default_creds<T>(f: T) -> CredentialsV2<T> where T: Fn(&str, &str) -> bool {
    CredentialsV2 {
        username: "guest".to_owned(),
        password: "Password123#".to_owned(),
        validator: f
    }
}

// Function returning a closure
// Error: min_len does not live long enough
// This closure captures min_len by reference, however the function is returned
// by the function while min_len gets dropped at the end of the function
// We can use the 'move' keyword to force the closure to take ownership of min_len.

// Using the 'impl' trait syntax takes advantage of static dispatch
// This syntax is ony available in simple clases like this one here
// when we are returning one closure
fn get_password_validator_v1(min_len: usize) -> impl Fn(&str, &str) -> bool {
    let validator = move |_: &str,  password: &str| -> bool {
        password.len() >= min_len
    };
    validator
}

// Error: Not two closures even if identical have the same type
// Consider boxing your closure and/or using it as a trait object
// We have to use dynamic dispatch via trait objects (Box smart pointer)
fn get_password_validator_v2(min_len: usize, special_char: bool) -> Box<dyn Fn(&str, &str) -> bool> {
    if special_char {
        let validator = move |_: &str,  password: &str| -> bool {
            password.len() >= min_len &&
            password.contains(['!', '@', '#', '$', '%', '&', '*'])
        };
        Box::new(validator)
    } else {
        let validator = move |_: &str,  password: &str| -> bool {
            password.len() >= min_len
        };
        Box::new(validator)
    }
}