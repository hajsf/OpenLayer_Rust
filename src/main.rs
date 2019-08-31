#![feature(proc_macro_hygiene, decl_macro)]
#![feature(const_vec_new)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
use rocket_contrib::{serve::StaticFiles, templates::Template};
use std::{thread, sync::Mutex};
mod sockets;
mod socket_handler; mod routes; mod models; mod databases;
use crate::routes::{index, submit_task};
use self::models::{client::Client, truck::Truck};
use crate::databases::main_connection::PgDb1;

lazy_static! {
    static ref CLIENTS: Mutex<Vec<Truck>> = Mutex::new(vec![]);
}

fn main() {
    let ws = ws::WebSocket::new(
        |out| Client { out }).unwrap();

    thread::spawn(|| { ws.listen("127.0.0.1:8080").unwrap(); });

    rocket::ignite()
    .mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
    .mount("/", routes![index::index, submit_task::submit_task])
    .attach(Template::fairing())
    .attach(PgDb1::fairing())
    .launch();
}
