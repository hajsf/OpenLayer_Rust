use crate::models::client::Client;

use crate::CLIENTS;
use crate::models::{truck::Truck};
use ws::{Result, Handshake};

impl Client {
    pub fn handle_on_open(&mut self, _: Handshake) -> Result<()> {
        let truck = Truck {
            socket: self.out.clone(),
            unique_id: "".to_string()
        };
        println!("before opening: {}", CLIENTS.lock().unwrap().len());
        CLIENTS.lock().unwrap().push(truck);
        println!("before opening: {}", CLIENTS.lock().unwrap().len());
        println!("called {:#?}", CLIENTS.lock().unwrap());
        Ok(())
    }
}
