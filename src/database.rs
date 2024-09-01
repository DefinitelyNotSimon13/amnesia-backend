use crate::reminder::Reminder;
use color_eyre::Result;
use mongodb::{Client, Collection, Database};
use tracing::{info, instrument};

#[derive(Debug)]
pub struct DbConnection<T: Send + Sync> {
    pub client: Client,
    pub database: Database,
    pub collection: Collection<T>,
}

impl<T: Send + Sync> DbConnection<T> {
    #[instrument(skip(connection_string))]
    pub async fn connect(
        connection_string: &str,
        database: &str,
        collection: &str,
    ) -> Result<DbConnection<T>> {
        let client = Client::with_uri_str(connection_string).await?;

        let database = client.database(database);
        let collection: Collection<T> = database.collection(collection);

        info!("Conncted to database");

        Ok(DbConnection {
            client,
            database,
            collection,
        })
    }
}
