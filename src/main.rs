use micorm::Document;
use serde::{Deserialize, Serialize};
use micorm_derive::Document;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, Document)]
#[micorm(db(database = "invex", collection = "test"))]
pub struct Test {
    #[serde(rename = "_id")]
    id: Option<Uuid>
}
fn main() {}