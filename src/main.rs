use lambda_runtime::{service_fn, Error, LambdaEvent};
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput, GetItemInput};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error as StdError;

#[derive(Deserialize)]
struct Request {
    command: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    command: String,
    count: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .init();

    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;

    // Example call to print_db_contents (might want to do this based on specific commands)
    // Note: In a real scenario, you might want to call this under specific conditions or in a different part of your application
    // print_db_contents().await?;

    Ok(())
}

async fn log_command(
    command: String,
) -> Result<i64, Box<dyn StdError + Send + Sync>> {
    let client = DynamoDbClient::new(Region::default());
    let mut count = 1;

    // Check if the "command" key already exists
    let get_request = GetItemInput {
        table_name: "my-table".to_string(),
        key: {
            let mut map = HashMap::new();
            map.insert(
                "command".to_string(),
                AttributeValue {
                    s: Some(command.clone()),
                    ..Default::default()
                },
            );
            map
        },
        ..Default::default()
    };

    let get_response = client.get_item(get_request).await?;

    // If the "command" key exists, increment its value. Otherwise, insert a default value of 1.
    let item = if let Some(mut item) = get_response.item {
        if let Some(count_attr) = item.get_mut("count") {
            if let Some(n) = &count_attr.n {
                count = n.parse::<i64>().unwrap_or(0) + 1;
                count_attr.n = Some(count.to_string());
            }
        }
        item
    } else {
        let mut new_item = HashMap::new();
        new_item.insert(
            "command".to_string(),
            AttributeValue {
                s: Some(command.clone()),
                ..Default::default()
            },
        );
        new_item.insert(
            "count".to_string(),
            AttributeValue {
                n: Some("1".to_string()),
                ..Default::default()
            },
        );
        new_item
    };

    let put_request = PutItemInput {
        table_name: "my-table".to_string(),
        item,
        ..Default::default()
    };

    client.put_item(put_request).await?;
    Ok(count)
}

async fn my_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let command = event.payload.command.clone();
    let request_id = event.context.request_id.clone();

    // Log the command to DynamoDB (assumes this should happen for all commands, adjust as needed)
    let count = match log_command(command.clone()).await {
        Ok(count) => count,
        Err(e) => return Err(lambda_runtime::Error::from(e.to_string())),
    };

    let resp = Response {
        req_id: request_id,
        command,
        count,
    };

    Ok(resp)
}