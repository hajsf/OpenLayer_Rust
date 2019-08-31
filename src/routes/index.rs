
use crate::databases::main_connection::PgDb1;
use crate::models::device::Device;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
pub fn index(conn: PgDb1) -> Template {
    let context = HashMap::<String, String>::new();
    for row in &conn.query("select id, data from devices where data->>'name' = $1",
        &[&"Hasan"]).unwrap() {
        let device = Device {
            id: row.get(0),
            data: row.get(1)
        };
        println!("Again Found device {}: {}", device.id, device.data);
    }
    Template::render("index", context)
}

/*
let devices = &conn().query("select id, data from devices where data->>'name' = $1",
    &[&"Hasan"]).
expect("Could'n read table");
let d_id: Uuid = devices.get(0).get(0);
let d_user: JSON = devices.get(0).get(1);
println!("id: {} and date: {}",d_id, d_user);
*/
