use lgr_multi_mod::auth_utils::models::Credentials;

fn main() {
    println!("Multi-Module");

    let creds = Credentials {
        username: "letsgetrusty".to_owned(),
        password: "password1234".to_owned(),
    };
    println!(" > main");
    lgr_multi_mod::authenticate(creds);
}
