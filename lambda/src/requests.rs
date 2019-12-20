use crate::types::Diagnostic;
use anyhow::Error;
use http::{HeaderMap, Method, Request, Response, Uri};
use hyper::Body;
use serde::Serialize;
use std::str::FromStr;

pub(crate) trait IntoRequest {
    fn into_req(self) -> Result<Request<Body>, Error>;
}

pub(crate) trait IntoResponse {
    fn into_rsp(self) -> Result<Response<Body>, Error>;
}

//   /runtime/invocation/next
#[derive(Debug, PartialEq)]
pub(crate) struct NextEventRequest;

impl IntoRequest for NextEventRequest {
    fn into_req(self) -> Result<Request<Body>, Error> {
        let req = Request::builder()
            .method(Method::GET)
            .uri(Uri::from_static("/runtime/invocation/next"))
            .body(Body::empty())?;
        Ok(req)
    }
}

#[derive(Debug, PartialEq)]
pub struct NextEventResponse<'a> {
    // lambda-runtime-aws-request-id
    pub request_id: &'a str,
    // lambda-runtime-deadline-ms
    pub deadline: u64,
    // lambda-runtime-invoked-function-arn
    pub arn: &'a str,
    // lambda-runtime-trace-id
    pub trace_id: &'a str,
    // the actual body,
    pub body: Vec<u8>,
}

impl<'a> IntoResponse for NextEventResponse<'a> {
    fn into_rsp(self) -> Result<Response<Body>, Error> {
        let rsp = Response::builder()
            .header("lambda-runtime-aws-request-id", self.request_id)
            .header("lambda-runtime-deadline-ms", self.deadline)
            .header("lambda-runtime-invoked-function-arn", self.arn)
            .header("lambda-runtime-trace-id", self.trace_id)
            .body(Body::from(self.body))?;
        Ok(rsp)
    }
}
#[test]
fn test_next_event_request() {
    let req = NextEventRequest;
    let req = req.into_req().unwrap();
    assert_eq!(req.method(), Method::GET);
    assert_eq!(req.uri(), &Uri::from_static("/runtime/invocation/next"));
}

// /runtime/invocation/{AwsRequestId}/response
pub(crate) struct EventCompletionRequest<'a, T> {
    pub(crate) request_id: &'a str,
    pub(crate) body: T,
}

impl<'a, T> IntoRequest for EventCompletionRequest<'a, T>
where
    T: for<'serialize> Serialize,
{
    fn into_req(self) -> Result<Request<Body>, Error> {
        let uri = format!("/runtime/invocation/{}/response", self.request_id);
        let uri = Uri::from_str(&uri)?;
        let body = serde_json::to_vec(&self.body)?;
        let body = Body::from(body);

        let req = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .body(body)?;
        Ok(req)
    }
}

#[test]
fn test_event_completion_request() {
    let req = EventCompletionRequest {
        request_id: "id",
        body: "hello, world!",
    };
    let req = req.into_req().unwrap();
    let expected = Uri::from_static("/runtime/invocation/id/response");
    assert_eq!(req.method(), Method::POST);
    assert_eq!(req.uri(), &expected);
}

// /runtime/invocation/{AwsRequestId}/error
pub(crate) struct EventErrorRequest<'a> {
    pub(crate) request_id: &'a str,
    pub(crate) diagnostic: Diagnostic,
}

impl<'a> IntoRequest for EventErrorRequest<'a> {
    fn into_req(self) -> Result<Request<Body>, Error> {
        let uri = format!("/runtime/invocation/{}/error", self.request_id);
        let uri = Uri::from_str(&uri)?;
        let body = serde_json::to_vec(&self.diagnostic)?;
        let body = Body::from(body);

        let req = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header("lambda-runtime-function-error-type", "unhandled")
            .body(body)?;
        Ok(req)
    }
}

#[test]
fn test_event_error_request() {
    let req = EventErrorRequest {
        request_id: "id",
        diagnostic: Diagnostic {
            error_type: "InvalidEventDataError".to_string(),
            error_message: "Error parsing event data".to_string(),
        },
    };
    let req = req.into_req().unwrap();
    let expected = Uri::from_static("/runtime/invocation/id/error");
    assert_eq!(req.method(), Method::POST);
    assert_eq!(req.uri(), &expected);
}

// /runtime/init/error
struct InitErrorRequest;

impl IntoRequest for InitErrorRequest {
    fn into_req(self) -> Result<Request<Body>, Error> {
        let uri = format!("/runtime/init/error");
        let uri = Uri::from_str(&uri)?;

        let req = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header("lambda-runtime-function-error-type", "unhandled")
            .body(Body::empty())?;
        Ok(req)
    }
}

#[test]
fn test_init_error_request() {
    let req = InitErrorRequest;
    let req = req.into_req().unwrap();
    let expected = Uri::from_static("/runtime/init/error");
    assert_eq!(req.method(), Method::POST);
    assert_eq!(req.uri(), &expected);
}
