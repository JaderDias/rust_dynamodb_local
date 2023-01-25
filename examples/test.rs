use aws_lambda_events::apigw::ApiGatewayV2httpRequest;
use aws_lambda_events::apigw::ApiGatewayV2httpResponse;
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::collections::HashMap;
use aws_lambda_events::encodings::Body;
use aws_sdk_dynamodb::model::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};

type TestCases = Vec<TestCase>;

#[derive(Deserialize)]
struct TestCase {
    request: ApiGatewayV2httpRequest,
    request_body: Option<Box<serde_json::value::RawValue>>,
    expected_response: ApiGatewayV2httpResponse,
    expected_body: Option<serde_json::Value>,
    expected_db_key: Option<String>,
    expected_db_item: Option<HashMap<String, String>>,
}

#[tokio::main]
async fn main() {
    let db_client = rust_lambda::dynamodb::get_client().await;
    create_table_if_not_exists(&db_client).await;
    let paths = std::fs::read_dir("./test-cases").unwrap();

    let http_client = reqwest::Client::new();

    for path in paths {
        let path_value = path.unwrap().path();
        println!("testing {}", path_value.to_str().unwrap());
        let file = File::open(path_value).unwrap();
        let mut file_deserializer = serde_json::Deserializer::from_reader(file);
        let test_cases = TestCases::deserialize(&mut file_deserializer).unwrap();
        for mut test in test_cases {
            match &test.request_body {
                Some(body) => {
                    test.request.body = Some(body.get().to_owned());
                }
                None => {}
            }

            let request = json!(test.request);
            let res = http_client
                .post("http://localhost:8080/2015-03-31/functions/function/invocations")
                .body(request.to_string())
                .send()
                .await
                .unwrap();
            assert_eq!(res.status(), 200);
            let response_text = &res.text().await.unwrap();
            let mut response: ApiGatewayV2httpResponse = serde_json::from_str(response_text)
                .map_err(|e| panic!("error parsing response: {}, response: {}", e, response_text))
                .unwrap();
            assert_body_matches(&mut response, &test.expected_body);
            assert_eq!(response, test.expected_response);
            assert_db_item(&test, &db_client).await;
        }
    }
}

async fn assert_db_item(test: &TestCase, db_client: &aws_sdk_dynamodb::Client) {
    match &test.expected_db_key {
        Some(key) => {
            let get_item_output = rust_lambda::dynamodb::get_item(
                &db_client,
                "table-name",
                key.as_str(),
            ).await.unwrap();
            let expected_item = test.expected_db_item.as_ref().unwrap();
            let actual_item = get_item_output.item().unwrap();
            for (key, value) in &*actual_item {
                match value {
                    aws_sdk_dynamodb::model::AttributeValue::S(s) => {
                        let expected_value = expected_item.get(key).unwrap();
                        assert_eq!(s.to_owned(), *expected_value);
                    }
                    _ => {
                        assert!(false);
                    }
                }
            }
        }
        None => {}
    }
}

fn assert_body_matches(actual: &mut ApiGatewayV2httpResponse, expected: &Option<serde_json::Value>) {
    match &actual.body {
        Some(b) => {
            match b {
                Body::Empty => {
                    assert!(expected.is_none())
                }
                Body::Text(actual_body) => {
                    match actual_body.as_str() {
                        "" =>
                            assert!(expected.is_none()),
                        _ => {
                            let expected_body = serde_json::to_string(&expected).unwrap();
                            assert_eq!(actual_body, &expected_body);
                            actual.body = None;
                        }
                    }
                }
                Body::Binary(_) => {
                    assert!(false)
                }
            }
        }
        None => {
            assert!(expected.is_none())
        }
    }
}

async fn table_exists(client: &aws_sdk_dynamodb::Client, table: &str) -> bool {
    let table_list = client.list_tables().send().await.unwrap();
    table_list
        .table_names()
        .as_ref()
        .unwrap()
        .contains(&table.into())
}

async fn create_table_if_not_exists(client: &aws_sdk_dynamodb::Client) {
    let table_name = "table-name";
    if table_exists(client, table_name).await {
        return;
    }

    let a_name: String = rust_lambda::dynamodb::PARTITION_KEY_NAME.to_owned();

    let ad = AttributeDefinition::builder()
        .attribute_name(&a_name)
        .attribute_type(ScalarAttributeType::S)
        .build();

    let ks = KeySchemaElement::builder()
        .attribute_name(&a_name)
        .key_type(KeyType::Hash)
        .build();

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(10)
        .write_capacity_units(5)
        .build();

    client
        .create_table()
        .table_name(table_name)
        .key_schema(ks)
        .attribute_definitions(ad)
        .provisioned_throughput(pt)
        .send()
        .await
        .unwrap();
}