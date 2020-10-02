// Generated with assemblylift-cli 0.1.0

extern crate asml_awslambda;

use asml_core::GuestCore;
use asml_awslambda::*;

use asml_iomod_dynamodb::{structs, *};

macro_rules! http_ok {
    ($response:ident) => {
        AwsLambdaClient::success(serde_json::to_string(
            &ApiGatewayResponse::ok(serde_json::to_string(&$response).unwrap(), None)).unwrap());
    }
}

handler!(context: LambdaContext, async {
    let event = context.event;
    AwsLambdaClient::console_log(format!("Read event: {:?}", event));

    let input: structs::ListTablesInput = Default::default();
    let response = list_tables(input).await;

    http_ok!(response);
});
