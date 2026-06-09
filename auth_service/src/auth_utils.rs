pub fn login(cred: models::Credentials) {
    crate::database::get_user();
}

pub mod models;
