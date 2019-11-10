#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;
extern crate mongodb;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_mongodb;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate ws;

use std::sync::{Arc, Mutex};

use dotenv::dotenv;
use rocket::{Request, Rocket};
use server::Server;
use std::thread;
use ws::listen;

mod mongo_connection;
mod tasks;
mod items;
mod sessions;
mod accounts;
mod server;
mod api_key;

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(400)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

pub fn rocket() -> Rocket {
    dotenv().ok();
    let server = Arc::new(Mutex::new(Server { out: None }));
    let clone_server = server.clone();

    // Starting WebSocket server responsible of the notification of updates(POST/PUT/DELETE) to clients
    thread::spawn(move || {
        listen(format!("{}:{}", String::from("127.0.0.1"), 3012), |out| {
            let mut lock = clone_server.try_lock();
            if let Ok(ref mut server) = lock {
                **server = Server {
                    out: Some(out.clone()),
                };
            } else {
                println!("try_lock failed");
            }
            |_| Ok(())
        })
        .unwrap()
    });

    let clone_server = server.clone();
    rocket::ignite()
        .register(catchers![internal_error, not_found])
        .manage(mongo_connection::init_pool())
        .manage(clone_server)
        .mount(
            "/tasks",
            routes![
                tasks::handler::all,
                tasks::handler::page,
                tasks::handler::get,
                tasks::handler::post,
                tasks::handler::put,
                tasks::handler::delete,
                tasks::handler::delete_all,
                tasks::handler::count
            ],
        )
        .mount(
            "/items",
            routes![
                items::handler::all,
                items::handler::page,
                items::handler::get,
                items::handler::post,
                items::handler::put,
                items::handler::delete,
                items::handler::delete_all,
                items::handler::count
            ],
        )
        .mount(
            "/sessions",
            routes![
                sessions::handler::all,
                sessions::handler::page,
                sessions::handler::get,
                sessions::handler::post,
                sessions::handler::put,
                sessions::handler::delete,
                sessions::handler::delete_all,
                sessions::handler::count
            ],
        )
        .mount(
            "/accounts",
            routes![
                accounts::handler::all,
                accounts::handler::page,
                accounts::handler::get,
                accounts::handler::post,
                accounts::handler::put,
                accounts::handler::delete,
                accounts::handler::delete_all,
                accounts::handler::count
            ],
        )
}
