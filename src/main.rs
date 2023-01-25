use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

mod dynamodb;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code examples in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub async fn handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    match event.raw_http_path().as_str() {
        "/api/v1/statuses" => {
           // let dynamodb_client =  dynamodb::get_client().await;
            let resp = Response::builder()
                .status(200)
                .body("".into())
                .map_err(Box::new)?;
            Ok(resp)
        }
        "/nodeinfo/2.0" => {
            let resp = Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body("{\"openRegistrations\":true,\"protocols\":[\"activitypub\"],\"services\":{\"inbound\":[],\"outbound\":[]},\"software\":{\"name\":\"rust_lambda\",\"version\":1},\"usage\":{\"localPosts\":1,\"users\":{\"total\":1}},\"version\":2.0}".into())
                .map_err(Box::new)?;
            Ok(resp)
        }
        _ => {
            let resp = Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body("".into())
                .map_err(Box::new)?;
            Ok(resp)
        }
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