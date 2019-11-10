use crate::mongo_connection::Conn;
use crate::accounts;
use crate::server::Server;
use crate::api_key::ApiKey;
use mongodb::{doc, error::Error, oid::ObjectId, coll::options::FindOptions};
use accounts::Account;
use rocket::{http::Status, State};
use rocket_contrib::json::Json;
use serde_json::json;
use std::{
    sync::{Arc, Mutex},
};

fn error_status(error: Error) -> Status {
    match error {
        Error::CursorNotFoundError => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/")]
pub fn all(connection: Conn) -> Result<Json<Vec<Account>>, Status> {
    match accounts::repository::all(None, &connection) {
        Ok(res) => Ok(Json(res.accounts)),
        Err(err) => Err(error_status(err)),
    }
}

// TODO: wouldn't it be better to implement FromFormValue for FindOptions ?
// https://api.rocket.rs/v0.4/rocket/request/trait.FromFormValue.html
#[get("/<skip>/<batch>")]
pub fn page(skip: Option<String>, batch: Option<String>, connection: Conn) -> Result<Json<accounts::repository::AccountsItemsLeft>, Status> {
    let mut options = FindOptions::new();
    options.skip = Some(skip.unwrap().parse::<i64>().expect("Na"));
    options.limit = Some(batch.unwrap().parse::<i64>().expect("Na"));
    match accounts::repository::all(Some(options), &connection) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(error_status(err)),
    }
}

#[get("/<id>")]
pub fn get(id: String, connection: Conn) -> Result<Json<Account>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match accounts::repository::get(res, &connection) {
            Ok(res) => Ok(Json(res.unwrap())),
            Err(err) => Err(error_status(err)),
        },
        Err(_) => Err(error_status(Error::DefaultError(String::from(
            "Couldn't parse ObjectId",
        )))),
    }
}

#[post("/", format = "application/json", data = "<accounts>")]
pub fn post(
    accounts: Json<Account>,
    connection: Conn,
    server: State<Arc<Mutex<Server>>>,
    _key: ApiKey
) -> Result<Json<Account>, Status> {
    match accounts::repository::insert(accounts.into_inner(), &connection) {
        Ok(res) => {
            if !server.inner().lock().unwrap().out.is_none() {
                // println!("Broadcast POST");
                let msg = json!({
                    "protocol": "POST".to_owned(),
                    "data": &res
                });
                // inner() get the thing inside State, then lock mutex, unwrap ...
                // We send the serialized data
                server
                    .inner()
                    .lock()
                    .unwrap()
                    .out
                    .as_ref()
                    .unwrap()
                    .broadcast(serde_json::to_string(&msg).unwrap())
                    .expect("Failed to broadcast");
            } else {
                // println!("No clients connected");
            }
            Ok(Json(res))
        }
        Err(err) => Err(error_status(err)),
    }
}

#[put("/<id>", format = "application/json", data = "<accounts>")]
pub fn put(
    id: String,
    accounts: Json<Account>,
    connection: Conn,
    server: State<Arc<Mutex<Server>>>,
    _key: ApiKey
) -> Result<Json<Account>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match accounts::repository::update(res, accounts.into_inner(), &connection) {
            Ok(res) => {
                if !server.inner().lock().unwrap().out.is_none() {
                    // println!("Broadcast PUT");
                    let msg = json!({
                        "protocol": "PUT".to_owned(),
                        "data": &res
                    });
                    // Get the Json<Account>> inside Created with .0 and converts it to string to send
                    // Broadcast the new account to all clients
                    // inner() get the thing inside State, then lock mutex, unwrap ...
                    // We send the serialized data
                    server
                        .inner()
                        .lock()
                        .unwrap()
                        .out
                        .as_ref()
                        .unwrap()
                        .broadcast(serde_json::to_string(&msg).unwrap())
                        .expect("Failed to broadcast");
                } else {
                    // println!("No clients connected");
                }
                Ok(Json(res))
            }
            Err(err) => Err(error_status(err)),
        },
        Err(_) => Err(error_status(Error::DefaultError(String::from(
            "Couldn't parse ObjectId",
        )))),
    }
}

#[delete("/<id>")]
pub fn delete(
    id: String,
    connection: Conn,
    server: State<Arc<Mutex<Server>>>,
    _key: ApiKey
) -> Result<Json<String>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match accounts::repository::delete(res, &connection) {
            Ok(_) => {
                if !server.inner().lock().unwrap().out.is_none() {
                    // println!("Broadcast DELETE");
                    let msg = json!({
                        "protocol": "DELETE".to_owned(),
                        "data": {
                            "id": &id
                        }
                    });
                    // Get the Json<Account>> inside Created with .0 and converts it to string to send
                    // Broadcast the new account to all clients
                    // inner() get the thing inside State, then lock mutex, unwrap ...
                    // We send the serialized data
                    server
                        .inner()
                        .lock()
                        .unwrap()
                        .out
                        .as_ref()
                        .unwrap()
                        .broadcast(serde_json::to_string(&msg).unwrap())
                        .expect("Failed to broadcast");
                } else {
                    // println!("No clients connected");
                }
                Ok(Json(id))
            }
            Err(err) => Err(error_status(err)),
        },
        Err(_) => Err(error_status(Error::DefaultError(String::from(
            "Couldn't parse ObjectId",
        )))),
    }
}

#[delete("/")]
pub fn delete_all(
    connection: Conn,
    server: State<Arc<Mutex<Server>>>,
    _key: ApiKey
) -> Result<Json<bool>, Status> {
    match accounts::repository::delete_all(&connection) {
        Ok(_) => {
            if !server.inner().lock().unwrap().out.is_none() {
                // println!("Broadcast DELETE ALL");
                let msg = json!({
                    "protocol": "DELETE_ALL".to_owned(),
                    "data": {} // TODO: think & improve the websocket protocol
                });
                // Get the Json<Account>> inside Created with .0 and converts it to string to send
                // Broadcast the new account to all clients
                // inner() get the thing inside State, then lock mutex, unwrap ...
                // We send the serialized data
                server
                    .inner()
                    .lock()
                    .unwrap()
                    .out
                    .as_ref()
                    .unwrap()
                    .broadcast(serde_json::to_string(&msg).unwrap())
                    .expect("Failed to broadcast");
            } else {
                // println!("No clients connected");
            }
            Ok(Json(true))
        }
        Err(err) => Err(error_status(err)),
    }
}

#[get("/count")]
pub fn count(connection: Conn) -> Result<Json<i64>, Status> {
    match accounts::repository::count(&connection) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(error_status(err)),
    }
}