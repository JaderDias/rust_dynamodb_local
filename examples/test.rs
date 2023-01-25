use aws_lambda_events::apigw::ApiGatewayV2httpRequest;
use aws_lambda_events::apigw::ApiGatewayV2httpResponse;
use aws_lambda_events::encodings::Body;
use aws_sdk_dynamodb::model::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};
use serde::Deserialize;
use serde_json::json;
use std::fs::File;

type TestCases = Vec<TestCase>;

const DB_URL: &str = "http://localhost:8000";
const TABLE_NAME: &str = "table_name";

#[derive(Deserialize)]
struct TestCase {
    request: ApiGatewayV2httpRequest,
    request_body: Option<Box<serde_json::value::RawValue>>,
    expected_response: ApiGatewayV2httpResponse,
    expected_body_json: Option<serde_json::Value>,
}

#[tokio::main]
async fn main() {
    let db_client = rust_lambda::dynamodb::get_local_client(DB_URL.to_owned()).await;
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
            let mut response: ApiGatewayV2httpResponse = serde_json::from_str(&response_text)
                .map_err(|e| panic!("error {} parsing response: {}", e, &response_text))
                .unwrap();
            assert_body_matches(&test, &mut response);
            assert_eq!(response, test.expected_response);
        }
    }
}

fn assert_body_matches(test: &TestCase, actual: &mut ApiGatewayV2httpResponse) {
    match &actual.body {
        Some(actual_body) => match actual_body {
            Body::Empty => {}
            Body::Text(actual_body_text) => match &test.expected_response.body {
                Some(expected_body) => match expected_body {
                    Body::Text(expected_body_text) => {
                        assert_eq!(actual_body_text, expected_body_text);
                        return;
                    }
                    _ => {
                        assert!(false)
                    }
                },
                None => match &test.expected_body_json {
                    Some(expected_body_value) => {
                        let expected_body_json =
                            serde_json::to_string(expected_body_value).unwrap();
                        assert_eq!(actual_body_text, &expected_body_json);
                        actual.body = None;
                        return;
                    }
                    None => {
                        assert!(false);
                    }
                },
            },
            Body::Binary(_) => {
                assert!(false)
            }
        },
        None => {}
    }

    assert!(test.expected_response.body.is_none());
    assert!(test.expected_body_json.is_none());
}

async fn table_exists(client: &aws_sdk_dynamodb::Client, table: &str) -> bool {
    let table_list = client.list_tables().send().await.unwrap();
    println!("tables {:?}", table_list);
    table_list
        .table_names()
        .as_ref()
        .unwrap()
        .contains(&table.into())
}

async fn create_table_if_not_exists(client: &aws_sdk_dynamodb::Client) {
    if table_exists(client, TABLE_NAME).await {
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
        .table_name(TABLE_NAME)
        .key_schema(ks)
        .attribute_definitions(ad)
        .provisioned_throughput(pt)
        .send()
        .await
        .unwrap();
}
