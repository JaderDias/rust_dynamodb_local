use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::error::{
    GetItemError,
    PutItemError,
};

pub const PARTITION_KEY_NAME: &str = "partition";

pub type GetItemResult = Result<aws_sdk_dynamodb::output::GetItemOutput, SdkError<GetItemError>>;
pub type PutItemResult = Result<aws_sdk_dynamodb::output::PutItemOutput, SdkError<PutItemError>>;

pub async fn get_item(
    client: &Client,
    dynamodb_table_name: &str,
    partition_key_value: &str,
) -> GetItemResult {
    let value = AttributeValue::S(partition_key_value.to_owned());
    client.get_item().table_name(dynamodb_table_name).key(PARTITION_KEY_NAME, value).send().await
}

pub async fn put_item(
    client: &Client,
    dynamodb_table_name: &str,
    partition_key_value: &str,
    values: std::collections::HashMap<String, AttributeValue>,
) -> PutItemResult {
    let value = AttributeValue::S(partition_key_value.to_owned());
    let mut table = client
        .put_item()
        .table_name(dynamodb_table_name)
        .item(PARTITION_KEY_NAME, value);
    for (key, value) in values {
        table = table.item(key, value);
    }
    table.send().await
}

pub async fn get_client() -> Client {
    match std::env::var("LOCAL_DYNAMODB_URL") {
        Ok(url) => {
            println!("Using local dynamodb at {}", url);
            get_local_client(url).await
        }
        Err(_) => {
            let config = aws_config::load_from_env().await;
            Client::new(&config)
        }
    }
}

pub async fn get_local_client(local_dynamodb_url: String) -> Client {
    let config = aws_config::from_env().region("us-east-1").load().await;
    let dynamodb_local_config = aws_sdk_dynamodb::config::Builder
    ::from(&config)
        .endpoint_url(local_dynamodb_url)
        .build();
    return Client::from_conf(dynamodb_local_config);
}