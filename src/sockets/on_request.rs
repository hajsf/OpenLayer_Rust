/*
use ws::Handler;
use crate::models::client::Client;

use ws::{Request, Response, Result};
use std::{fs};

impl Handler for Client {
    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        match req.resource() {
            "/ws" => Response::from_request(req),
            "/" => {
                let filename = "index.html";
                let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
                Ok(Response::new(200, "OK", contents.as_bytes().to_vec()))
            }
            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }
}
*/
