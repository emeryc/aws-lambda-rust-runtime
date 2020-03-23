extern crate lambda;
extern crate lambda_http;
extern crate tokio;

use http::Response;
use lambda::lambda;
use lambda_http::{IntoResponse, LambdaRequest, LambdaResponse, Request, RequestExt};
use log::{self, error};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda]
#[tokio::main]
async fn main(e: LambdaRequest<'_>) -> Result<LambdaResponse, Error> {
    let response: Request = e.into();
    // info!("{:?}", e);
    Ok(match response.query_string_parameters().get("first_name") {
        Some(first_name) => format!("Hello, {}!", first_name).into_response(),
        _ => {
            error!("Empty first name in request {}", lambda::context().aws_request_id);
            Response::builder()
                .status(400)
                .body("Empty first name".into())
                .expect("failed to render response")
        }
    });
    Ok(LambdaResponse::from_response(
        false,
        Response::builder().body("test".to_string()).unwrap(),
    ))
}
