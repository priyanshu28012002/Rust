#![allow(dead_code, unused_variables)]

pub fn authenticate(cred: auth_utils::models::Credentials) {
    if let database::Status::Connected = database::connect_to_database() {
        auth_utils::login(cred);
    };
}

pub mod auth_utils;
pub mod database;
// src/util.rs
// src/util/mod.rs
