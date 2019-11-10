#![allow(proc_macro_derive_resolution_fallback)]

pub mod handler;
pub mod repository;
use mongodb::bson;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    name: Option<String>,
    server: Option<String>,
    level: Option<i32>,
    xp: Option<f32>,
    kamas: Option<i32>,
    inventory: Option<String>, // TODO: vec struct item etc
    class: Option<String>, // enum or not ? code duplication with dofus-data ? better only keep string ?
    jobs: Option<String>,
    stats: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub username: String,
    pub password: String,
    pub characters: Option<std::vec::Vec<Character>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableAccount {
    pub username: String,
    pub password: String,
    pub characters: Option<std::vec::Vec<Character>>
}

impl InsertableAccount {
    fn from_account(account: Account) -> InsertableAccount {
        InsertableAccount {
            username: account.username,
            password: account.password,
            characters: account.characters
        }
    }
    
    fn assign_id(account: InsertableAccount, new_id: bson::oid::ObjectId) -> Account {
        Account {
            id: Some(new_id),
            username: account.username,
            password: account.password,
            characters: account.characters
        }
    }
}
