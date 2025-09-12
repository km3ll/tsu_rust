use bootcamp_p04::{Credentials, authenticate};

fn main() {
    let creds = Credentials {
        username: "ferris".to_owned(),
        password: "pass12".to_owned(),
    };
    authenticate(creds);
}
