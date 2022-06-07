#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use lettre::{ SmtpTransport };
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

#[async_std::main]
async fn main() {
    dotenv().ok();

    let _mailer = SmtpTransport::unencrypted_localhost();
    let _conn = connect_db();
}

pub fn connect_db() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MysqlConnection::establish(database_url.as_str())
        .expect(&format!("Error connecting to {}", database_url))
}