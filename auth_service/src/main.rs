use auth_service::authenticate;
use auth_service::auth_utils::models::Credentials;

fn main() {

    let cred = Credentials {
        username: String::from("Priyanshu"),
                password: String::from("12345678"),

    };
    authenticate(cred);
    println!("Hello, world!");
}
 