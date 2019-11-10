#![allow(proc_macro_derive_resolution_fallback)]
use crate::mongo_connection::Conn;
use crate::sessions::{InsertableSession, Session};
use crate::r2d2_mongodb::mongodb::db::ThreadedDatabase;
use mongodb::{bson, coll::results::DeleteResult, doc, error::Error, oid::ObjectId, coll::options::FindOptions};

const COLLECTION: &str = "sessions";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionsItemsLeft {
    pub sessions: Vec<Session>,
    pub sessions_left: i64
}

/// Returns sessions and sessions left
/// sessions left are computed as follow: total sessions - skip option - batch size (found)
/// 
/// # Arguments
///
/// * `options` - Options for the query
///
pub fn all(options: Option<FindOptions>, connection: &Conn) -> Result<SessionsItemsLeft, Error> {
    let cursor = connection.collection(COLLECTION).find(None, options.clone()).unwrap();
    let mut sessions_left = count(connection).expect("");
    let sessions = cursor
        .map(|result| match result {
            Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(result_model) => Ok(result_model),
                Err(_) => Err(Error::DefaultError(String::from(""))),
            },
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<Session>, Error>>();
    match sessions {
        Ok(obj) => {
                match options {
                    Some(o) => sessions_left = sessions_left - o.skip.unwrap_or(0) - obj.len() as i64,
                    None => sessions_left = 0 // No options = we want to get it all
                }
                Ok(SessionsItemsLeft{sessions: obj.clone(), sessions_left: sessions_left})
            },
        Err(err) => Err(err)
    }
}

pub fn get(id: ObjectId, connection: &Conn) -> Result<Option<Session>, Error> {
    match connection
        .collection(COLLECTION)
        .find_one(Some(doc! {"_id": id}), None)
    {
        Ok(db_result) => match db_result {
            Some(result_doc) => match bson::from_bson(bson::Bson::Document(result_doc)) {
                Ok(result_model) => Ok(Some(result_model)),
                Err(_) => Err(Error::DefaultError(String::from(
                    "Failed to create reverse BSON",
                ))),
            },
            None => Ok(None),
        },
        Err(err) => Err(err),
    }
}

pub fn insert(sessions: Session, connection: &Conn) -> Result<Session, Error> {
    let insertable = InsertableSession::from_session(sessions.clone());
    match bson::to_bson(&insertable) {
        Ok(model_bson) => match model_bson {
            bson::Bson::Document(model_doc) => {
                match connection
                    .collection(COLLECTION)
                    .insert_one(model_doc, None)
                {
                    Ok(res) => match res.inserted_id {
                        Some(res) => match bson::from_bson(res) {
                            Ok(res) => Ok(InsertableSession::assign_id(insertable, res)),
                            Err(_) => Err(Error::DefaultError(String::from("Failed to read BSON"))),
                        },
                        None => Err(Error::DefaultError(String::from("None"))),
                    },
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::DefaultError(String::from(
                "Failed to create Document",
            ))),
        },
        Err(_) => Err(Error::DefaultError(String::from("Failed to create BSON"))),
    }
}

pub fn update(id: ObjectId, sessions: Session, connection: &Conn) -> Result<Session, Error> {
    let mut new_session = sessions.clone();
    new_session.id = Some(id.clone());
    match bson::to_bson(&new_session) {
        Ok(model_bson) => match model_bson {
            bson::Bson::Document(model_doc) => {
                match connection.collection(COLLECTION).replace_one(
                    doc! {"_id": id},
                    model_doc,
                    None,
                ) {
                    Ok(_) => Ok(new_session),
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::DefaultError(String::from(
                "Failed to create Document",
            ))),
        },
        Err(_) => Err(Error::DefaultError(String::from("Failed to create BSON"))),
    }
}

pub fn delete(id: ObjectId, connection: &Conn) -> Result<DeleteResult, Error> {
    connection
        .collection(COLLECTION)
        .delete_one(doc! {"_id": id}, None)
}

pub fn delete_all(connection: &Conn) -> Result<(), Error> {
    connection.collection(COLLECTION).drop()
}

pub fn count(connection: &Conn) -> Result<i64, Error> {
    connection.collection(COLLECTION).count(None, None)
}