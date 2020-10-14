// Generated with assemblylift-cli 0.1.0

extern crate asml_awslambda;

use asml_core::GuestCore;
use asml_awslambda::*;

use asml_iomod_dynamodb::{structs, *};

handler!(context: LambdaContext, async {
    let event = context.event;
    AwsLambdaClient::console_log(format!("Read event: {:?}", event));

    let input: structs::ListTablesInput = Default::default();
    match list_tables(input).await {
        Ok(response) => http_ok!(response),
        Err(why) => http_error!(why.to_string())
    }
});
