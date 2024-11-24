use std::{cell::OnceCell, error::Error, str::FromStr};

use async_trait::async_trait;
use mongodb::{bson::{self, doc}, error, Collection, Cursor, Database};
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

pub const CLIENT: OnceCell<Client> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct Client (mongodb::Client);

impl Client {
    pub fn client(&self) -> mongodb::Client {
        self.0.clone()
    }

    pub async fn activate<S: AsRef<str>, D: AsRef<str>>(uri: S) -> Result<(), Box<dyn Error>> {
        let client = mongodb::Client::with_uri_str(uri).await?;
        Ok(CLIENT.set(Client(client)).expect("Already set global client"))
    }

    pub fn database(&self, name: String) -> Database {
        self.client().database(&name)
    }

    pub fn collection<T: Serialize + DeserializeOwned + Send + Sync, D: AsRef<str>, C: AsRef<str>>(&self, database: D, collection: C) -> Collection<T> {
        self.database(database.as_ref().to_string()).collection::<T>(collection.as_ref())
    }
}

#[async_trait]
pub trait Document: Serialize + DeserializeOwned + Send + Sync {
    fn collection_name() -> String;
    fn database_name() -> String;
    fn get_id(&self) -> Option<Uuid>;
    fn set_id(&mut self, id: Uuid) -> ();

    fn id(&mut self) -> Uuid {
        if let Some(id) = self.get_id() {
            id
        } else {
            let new_id = Uuid::new_v4();
            self.set_id(new_id);
            new_id
        }
    }

    fn collection() -> Collection<Self> {
        Self::client().collection::<Self, String, String>(Self::database_name(), Self::collection_name())
    }

    fn client() -> Client {
        CLIENT.get().expect("Global client is unset").clone()
    }

    async fn find(filter: bson::Document) -> error::Result<Cursor<Self>> {
        Self::collection().find(filter).await
    }

    async fn find_one(filter: bson::Document) -> error::Result<Option<Self>> {
        Self::collection().find_one(filter).await
    }

    fn _collection(&self) -> Collection<Self> {
        Self::collection()
    }

    async fn save(&mut self) -> error::Result<Option<Uuid>> {
        let result = self._collection().replace_one(doc! {"_id": self.id().to_string()}, self).upsert(true).await?;
        if let Some(id) = result.upserted_id {
            if let Some(id_str) = id.as_str() {
                if let Ok(uuid) = Uuid::from_str(id_str) {
                    return Ok(Some(uuid));
                }
            }
        }
        Ok(None)
    }

    async fn delete(&mut self) -> error::Result<()> {
        self._collection().delete_one(doc! {"_id": self.id().to_string()}).await?;
        Ok(())
    }
}