use lambda_runtime::{handler_fn, Context, Error};
use rusoto_core::{HttpClient, Region};
use rusoto_credential::AwsCredentials;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
use serde_json::{json, Value};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let time = Instant::now();
    println!("{:?}", time);
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

// TODO move to dynomite???

async fn func(event: Value, _: Context) -> Result<Value, Error> {
    let credentialProvider = AwsCredentials::new(
        std::env::var("ID").unwrap(),
        std::env::var("SECRET").unwrap(),
        None,
        None,
    );
    //let client = DynamoDbClient::new_with(HttpClient::new(), credentialProvider, Region::UsEast1);

    //let items = client.batch_get_item()

    let first_name = event["firstName"].as_str().unwrap_or("world");

    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}
