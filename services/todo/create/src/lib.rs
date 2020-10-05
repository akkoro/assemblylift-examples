extern crate asml_awslambda;

use serde::{Serialize, Deserialize};

use asml_core::GuestCore;
use asml_awslambda::*;

use asml_iomod_crypto::uuid4;
use asml_iomod_dynamodb::{structs, structs::AttributeValue, *};

macro_rules! http_ok {
    ($response:ident) => {
        AwsLambdaClient::success(serde_json::to_string(
            &ApiGatewayResponse::ok(serde_json::to_string(&$response).unwrap(), None)).unwrap());
    }
}

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
            input.item.insert(String::from("date"), val!(S => content.date));

            let response = put_item(input).await;
            http_ok!(response);
        }

        None => {
            let response = "ERROR";
            http_ok!(response);
        }
    }
});

#[derive(Serialize, Deserialize)]
struct CreateTodoRequest {
    pub body: String,
    pub date: String,
}
