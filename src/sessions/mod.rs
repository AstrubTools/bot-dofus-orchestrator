#![allow(proc_macro_derive_resolution_fallback)]

pub mod handler;
pub mod repository;
use mongodb::bson;

pub struct MovementData {
    direction: Option<String>,
    sex: Option<bool>,
    colors: Option<String>,
    accessories: Option<String>,
    aura: Option<String>,
    emote: Option<String>,
    guild_name: Option<String>,
    cell: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub account_id: bson::oid::ObjectId,
    pub character: Option<String>,
    pub start_time: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableSession {
    pub account_id: bson::oid::ObjectId,
    pub character: Option<String>,
    pub start_time: Option<String>
}

impl InsertableSession {
    fn from_session(session: Session) -> InsertableSession {
        InsertableSession {
            account_id: session.account_id,
            character: session.character,
            start_time: session.start_time
        }
    }
    
    fn assign_id(session: InsertableSession, new_id: bson::oid::ObjectId) -> Session {
        Session {
            id: Some(new_id),
            account_id: session.account_id,
            character: session.character,
            start_time: session.start_time
        }
    }
}
