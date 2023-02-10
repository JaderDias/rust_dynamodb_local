use aws_lambda_events::apigw::ApiGatewayV2httpRequest;
use aws_lambda_events::apigw::ApiGatewayV2httpResponse;
use aws_lambda_events::encodings::Body;
use regex::Regex;
use serde::Deserialize;
use serde_json::{json, Value};
use std::env;
use std::fs::File;

type TestCases = Vec<TestCase>;

const UUID_PLACEHOLDER: &str = "abcdef1234567890abcdef1234567890";

#[derive(Deserialize)]
struct TestCase {
    request: ApiGatewayV2httpRequest,
    request_body_json: Option<Value>,
    expected_response: ApiGatewayV2httpResponse,
    expected_body_json: Option<Value>,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let test_target_url = &args[1];

    let paths = std::fs::read_dir("./test-cases").unwrap();

    let http_client = reqwest::Client::new();

    for path in paths {
        let path_value = path.unwrap().path();
        println!("Testing: {}", path_value.display());
        let file = File::open(path_value).unwrap();
        let mut file_deserializer = serde_json::Deserializer::from_reader(file);
        let test_cases = TestCases::deserialize(&mut file_deserializer).unwrap();
        let mut last_uuid = String::new();
        for mut test in test_cases {
            match &test.request_body_json {
                Some(body) => {
                    test.request.body = Some(serde_json::to_string(body).unwrap());
                }
                None => {}
            }

            let request = &test.request;
            println!(
                "{} {}",
                &request.request_context.http.method,
                request.raw_path.as_ref().unwrap()
            );
            let reqwest_response = http_client
                .post(test_target_url)
                .body(json!(request).to_string())
                .send()
                .await
                .unwrap();
            let actual_body_text = &reqwest_response.text().await.unwrap();
            let mut actual_response: ApiGatewayV2httpResponse =
                serde_json::from_str(&actual_body_text)
                    .map_err(|e| panic!("error {} parsing response: {}", e, &actual_body_text))
                    .unwrap();

            last_uuid = assert_body_matches_with_replacement(&test, &actual_body_text);

            assert_eq!(actual_response, test.expected_response);
        }
    }
}

fn assert_body_matches_with_replacement(test: &TestCase, actual_body_text: &String) -> String {
    let uuid_re = Regex::new(r"[a-f0-9]{32}").unwrap();
    if let Some(capture) = uuid_re.captures_iter(actual_body_text.as_str()).next() {
        let replaced_text = uuid_re
            .replace_all(actual_body_text, UUID_PLACEHOLDER)
            .to_string();
        assert_body_matches(test, &replaced_text);
        return capture[0].to_owned();
    }

    assert_body_matches(test, actual_body_text);
    String::new()
}

fn assert_body_matches(test: &TestCase, actual_body_text: &String) {
    match &test.expected_response.body {
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
                let actual_body_value: Value = serde_json::from_str(actual_body_text).unwrap();
                assert_eq!(&actual_body_value, expected_body_value);
                return;
            }
            None => {
                assert!(false);
            }
        },
    }

    assert!(test.expected_response.body.is_none());
    assert!(test.expected_body_json.is_none());
}
