extern crate core;

use crate::activitypub::object::Object;
use crate::dynamodb::{get_item, get_uuid, put_item};
use aws_sdk_dynamodb::model::AttributeValue;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

mod activitypub;
mod dynamodb;

fn bad_request() -> Result<Response<Body>, Error> {
    let resp = Response::builder()
        .status(400)
        .body("".into())
        .map_err(Box::new)?;
    Ok(resp)
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code examples in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub async fn handler(event: Request) -> Result<Response<Body>, Error> {
    let table_name = match std::env::var("DYNAMODB_TABLE") {
        Ok(t) => t,
        _ => {
            panic!("missing environment variable DYNAMODB_TABLE")
        }
    };
    let dynamodb_client = dynamodb::get_client().await;
    match event.raw_http_path().as_str() {
        "/api/v1/statuses" => match event.body() {
            Body::Text(body) => {
                let object: Object = serde_json::from_str(body).unwrap();
                let object_type = object.r#type.as_str();
                match object_type {
                    "Note" => {
                        let partition = get_uuid();
                        let content = object.content.unwrap();
                        let published = object.published.unwrap();
                        let sensitive = object.sensitive.unwrap();
                        let fields = std::collections::HashMap::from([
                            ("type".to_owned(), AttributeValue::S(object_type.to_owned())),
                            ("content".to_owned(), AttributeValue::S(content)),
                            ("published".to_owned(), AttributeValue::S(published)),
                            ("sensitive".to_owned(), AttributeValue::Bool(sensitive)),
                        ]);
                        put_item(
                            &dynamodb_client,
                            table_name.as_str(),
                            partition.as_str(),
                            fields,
                        )
                        .await
                        .unwrap();
                        let response = Response::builder()
                            .status(200)
                            .body(partition.to_string().into())
                            .map_err(Box::new)?;
                        Ok(response)
                    }
                    _ => bad_request(),
                }
            }
            _ => bad_request(),
        },
        "/nodeinfo/2.0" => {
            let resp = Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body("{\"openRegistrations\":true,\"protocols\":[\"activitypub\"],\"services\":{\"inbound\":[],\"outbound\":[]},\"software\":{\"name\":\"rust_lambda\",\"version\":1},\"usage\":{\"localPosts\":1,\"users\":{\"total\":1}},\"version\":2.0}".into())
                .map_err(Box::new)?;
            Ok(resp)
        }
        "/users/test_username/statuses/abcdef1234567890abcdef1234567890" => {
            let partition = get_uuid();
            let get_item_output =
                get_item(&dynamodb_client, table_name.as_str(), partition.as_str())
                    .await
                    .unwrap();
            let item = get_item_output.item.unwrap();
            let object: Object = serde_dynamo::from_item(item)?;
            let response_body = serde_json::json!(object).to_string();
            let resp = Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(response_body.into())
                .map_err(Box::new)?;
            Ok(resp)
        }
        _ => bad_request(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(handler)).await
}
