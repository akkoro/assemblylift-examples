extern crate asml_awslambda;

use serde::{Serialize, Deserialize};

use asml_core::GuestCore;
use asml_core_io::get_time;
use asml_awslambda::*;

use asml_iomod_crypto::uuid4;
use asml_iomod_dynamodb::{structs, structs::AttributeValue, *};

handler!(context: LambdaContext, async {
    let event: ApiGatewayEvent = context.event;
    match event.body {
        Some(content) => {
            let content: CreateTodoRequest = serde_json::from_str(&content).unwrap();

            let uuid = uuid4(()).await;

            let mut input: structs::PutItemInput = Default::default();
            input.table_name = String::from("todo-example");
            input.item = Default::default();
            input.item.insert(String::from("pk"), val!(S => uuid));
            input.item.insert(String::from("body"), val!(S => content.body));
            input.item.insert(String::from("timestamp"), val!(N => get_time()));

            match put_item(input).await {
                Ok(_) => {
                    let response = CreateTodoResponse { uuid };
                    http_ok!(response);
                }
                Err(why) => http_error!(why.to_string())
            }
        }

        None => {
            http_error!(String::from("missing request payload"));
        }
    }
});

#[derive(Serialize, Deserialize)]
struct CreateTodoRequest {
    pub body: String,
}

#[derive(Serialize, Deserialize)]
struct CreateTodoResponse{
    pub uuid: String,
}
