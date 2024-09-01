use amnesia_backend::install_color_eyre;
use amnesia_backend::reminder::Reminder;
use amnesia_backend::{database::DbConnection, setup_tracing};
use chrono::{DateTime, Utc};
use color_eyre::Result;
use dotenvy::dotenv;
use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing();
    install_color_eyre()?;

    dotenv().ok();

    let connection_string = std::env::var("CONNECTION_STRING")?;

    let db: DbConnection<Reminder> =
        DbConnection::connect(&connection_string, "amnesia", "reminders").await?;

    // let movie = dcollection
    //     .find_one(doc! { "title": "The Perils of Pauline" })
    //     .await?;

    println!("Hello there!");

    Ok(())
}
