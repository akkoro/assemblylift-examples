// Generated with assemblylift-cli 0.2.1

extern crate asml_awslambda;

use serde::{Serialize, Deserialize};

use asml_core::GuestCore;
use asml_awslambda::{*, AwsLambdaClient, LambdaContext};

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
            let content: DeleteTodoRequest = serde_json::from_str(&content).unwrap();

            let mut input: structs::DeleteItemInput = Default::default();
            input.table_name = String::from("todo-example");
            input.key = Default::default();
            input.key.insert(String::from("pk"), val!(S => content.uuid));
            input.key.insert(String::from("date"), val!(S => content.date));

            let response = delete_item(input).await;
            http_ok!(response);
        }

        None => {
            let response = "ERROR";
            http_ok!(response);
        }
    }
});

#[derive(Serialize, Deserialize)]
struct DeleteTodoRequest {
    pub uuid: String,
    pub date: String,
}
