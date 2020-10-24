// Generated with assemblylift-cli 0.2.1

extern crate asml_awslambda;

use serde::{Serialize, Deserialize};

use asml_core::GuestCore;
use asml_awslambda::{*, AwsLambdaClient, LambdaContext};

use asml_iomod_dynamodb::{structs, structs::AttributeValue, *};

handler!(context: LambdaContext, async {
    let event: ApiGatewayEvent = context.event;
    match event.body {
        Some(content) => {
            let content: DeleteTodoRequest = serde_json::from_str(&content).unwrap();

            let mut input: structs::DeleteItemInput = Default::default();
            input.table_name = String::from("todo-example");
            input.key = Default::default();
            input.key.insert(String::from("pk"), val!(S => content.uuid));
            input.key.insert(String::from("timestamp"), val!(N => content.timestamp));

            match delete_item(input).await {
                Ok(response) => http_ok!(response),
                Err(why) => http_error!(why.to_string())
            }
        }

        None => {
            http_error!(String::from("missing request payload"));
        }
    }
});

#[derive(Serialize, Deserialize)]
struct DeleteTodoRequest {
    pub uuid: String,
    pub timestamp: u64,
}
