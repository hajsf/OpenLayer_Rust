use ws::Handler;
use crate::models::client::Client;

use crate::CLIENTS;
use crate::models::{truck::Truck, app_action::AppAction, location::Location,
sqlmodel::SQLModel, confirmation::Confirmation, app_action::inspect};
use ws::{Message, Request, Response, Result, CloseCode, Handshake};
use std::{fs};
//mod crate::mod;
//use self::{CLIENTS, inspect, Confirmation};

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
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

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Server got message '{}'. ", msg);

        println!("before msg: {:#?}", CLIENTS.lock().unwrap());

        match msg {
            Message::Text(msg) => {

            if let Ok(loc) = serde_json::from_str::<Location>(&msg) {
                println!("Location!");
                match self.out.broadcast(msg.clone()) {
                    Ok(expr) =>  println!("broadcast sent {:?}", expr),
                    Err(e) => println!("broadcast faild {:?}", e),
                }
               // self.out.broadcast(msg.clone());
                CLIENTS.lock().unwrap().iter_mut()
                    .filter(|client| client.socket == self.out)
                    .for_each(|s| s.unique_id = loc.unique_id.clone());

            } else if let Ok(sql) = serde_json::from_str::<SQLModel>(&msg) {
                println!("Sql!",);
                println!("Transaction: {}", &sql.trx_id);


                let command = serde_json::to_string(&Confirmation {
                    command: inspect(AppAction::Confirmation),
                    trx_id: sql.trx_id.clone(),
                    updated: true
                }).unwrap();

                match self.out.send(command.clone()) {
                    Ok(expr) =>  println!("data sent {:?}", expr),
                    Err(e) => println!("data faild {:?}", e),
                }
              //  self.out.send(command.clone());

            } else {
                    println!("Unrecognized");
            };
                Ok(())
            },
            Message::Binary(_bin) => {
                Ok(())
            }
        }
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("before closing: {}", CLIENTS.lock().unwrap().len());
        CLIENTS.lock().unwrap().retain(|client| client.socket != self.out);
        println!("after closing: {}", CLIENTS.lock().unwrap().len());

        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }

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
