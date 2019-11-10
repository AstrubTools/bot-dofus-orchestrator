#![allow(proc_macro_derive_resolution_fallback)]

pub mod handler;
pub mod repository;
use mongodb::bson;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub item_id: String,
    pub stats: Option<String>,
    pub unit_price: Option<i32>,
    pub ten_price: Option<i32>,
    pub hundred_price: Option<i32> // TO improve everything
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableItem {
    pub item_id: String,
    pub stats: Option<String>,
    pub unit_price: Option<i32>,
    pub ten_price: Option<i32>,
    pub hundred_price: Option<i32> // TO improve everything
}

impl InsertableItem {
    fn from_item(item: Item) -> InsertableItem {
        InsertableItem {
            item_id: item.item_id,
            stats: item.stats,
            unit_price: item.unit_price,
            ten_price: item.ten_price,
            hundred_price: item.hundred_price,
        }
    }
    
    fn assign_id(item: InsertableItem, new_id: bson::oid::ObjectId) -> Item {
        Item {
            id: Some(new_id),
            item_id: item.item_id,
            stats: item.stats,
            unit_price: item.unit_price,
            ten_price: item.ten_price,
            hundred_price: item.hundred_price,
        }
    }
}
