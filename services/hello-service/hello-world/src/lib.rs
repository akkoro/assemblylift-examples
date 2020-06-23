// Generated with assemblylift-cli 0.1.0

extern crate asml_awslambda;

use direct_executor;
use asml_core::GuestCore;
use asml_awslambda::{*, AwsLambdaClient, LambdaContext};

use asml_awslambda_iomod::{structs, database::*};

handler!(context: LambdaContext, async {
    let event = context.event;
    AwsLambdaClient::console_log(format!("Read event: {:?}", event));

    let input: structs::ListTablesInput = Default::default();
    let response = aws_dynamodb_list_tables(input).await;

    AwsLambdaClient::success(format!("{:?}", response).to_string());
});
