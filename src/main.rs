#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;
extern crate dotenv;
use lettre::{ SmtpTransport };
use sqlx::mysql::MySqlPoolOptions;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let _mailer = SmtpTransport::unencrypted_localhost();
    let pool = MySqlPoolOptions::new().connect(dotenv!("DATABASE_URL")).await?;

    let q= sqlx::query("show tables;").execute(&pool).await?;
    eprintln!("{:?}", q);

    Ok(())
}