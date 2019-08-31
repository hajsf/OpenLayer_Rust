/*
use ws::Handler;
use crate::models::client::Client;

use crate::CLIENTS;
use crate::models::{ app_action::AppAction, location::Location,
sqlmodel::SQLModel, confirmation::Confirmation, app_action::inspect};
use ws::{Message, Result};

impl Handler for Client {
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
}
*/
