use crate::models::task::Task;
use crate::CLIENTS;
use rocket_contrib::json::{Json, JsonValue, JsonError};

#[post("/", data = "<task>")]
pub fn submit_task(task: Result<Json<Task>, JsonError>) -> JsonValue {
    match task {
        Ok(value) => {
            println!("task: {:?}", value.0);  // value is json{Task{}}, value.0 is Task{}
            let command = serde_json::to_string(&value.0).unwrap();

            CLIENTS.lock().unwrap().iter_mut()
                .filter(|client| client.unique_id == value.truck_id)
                .for_each(|client| {
                    println!("received msg from client: {:#?}", client);
                   // client.socket.send(command.clone());
                match client.socket.send(command.clone()) {
                    Ok(expr) =>  println!("broadcast sent {:?}", expr),
                    Err(e) => println!("broadcast faild {:?}", e),
                }
                }) ;
            json!({
                "status": "Success",
                "reason": format!("Received Json: {:?}", value.0)
            })
        },
        Err(JsonError::Io(e)) => json!({
                "status": "error",
                "reason": format!("I/O Error: {:?}", e)
            }),
        Err(JsonError::Parse(raw, e)) => json!({
                "status": "error",
                "reason": format!("{:?} is not valid JSON: {:?}", raw, e)
            })
    }
}
