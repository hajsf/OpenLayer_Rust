use crate::models::client::Client;

use crate::CLIENTS;
use ws::{CloseCode};

impl Client {
    pub fn handle_on_close(&mut self, code: CloseCode, reason: &str) {
        println!("before closing: {}", CLIENTS.lock().unwrap().len());
        CLIENTS.lock().unwrap().retain(|client| client.socket != self.out);
        println!("after closing: {}", CLIENTS.lock().unwrap().len());

        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}
