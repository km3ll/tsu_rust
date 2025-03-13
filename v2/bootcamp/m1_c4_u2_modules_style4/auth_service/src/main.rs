use auth_service::Credentials;

fn main() {
    let credentials = Credentials {
        username: "ferris".to_owned(),
        password: "password".to_owned(),
    };
    auth_service::authenticate(credentials);
}