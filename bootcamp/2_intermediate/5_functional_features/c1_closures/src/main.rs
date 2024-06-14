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

    println!("validate v1.1: {}", validate_credentials_v1(&creds_v1.username, &creds_v1.password));
    println!("validate v1.2: {}", validator_v2(&creds_v1.username, &creds_v1.password));
    println!("validate v2: {}", creds_v2.is_valid());
    println!("validate v3: {}", creds_v3.is_valid());
    println!("validate v4: {}", creds_v4.is_valid());
    
}

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

fn validate_credentials_v1(username: &str, password: &str) -> bool { 
    !username.is_empty() && !password.is_empty()
}