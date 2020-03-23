#![warn(missing_docs)]
//#![deny(warnings)]
//! Enriches the `lambda_runtime` crate with [http](https://github.com/hyperium/http)
//! types targeting ALB and API Gateway proxy events.
//!
//! Though ALB and API Gateway proxy events are separate Lambda triggers, they both share
//! similar shapes that contextually map to an http request handler. From a application perspective
//! the differences shouldn't matter. This crate
//! abstracts over both using standard [http](https://github.com/hyperium/http) types allowing
//! you to focus more on your application while giving you to the flexibility to
//! transparently use whichever http trigger suits your application's needs best.
//!
//! # Examples
//!
//! ```rust,no_run
//! use lambda_http::{lambda, IntoResponse, Request, RequestExt};
//! use lambda_runtime::{Context, error::HandlerError};
//!
//! fn main() {
//!     lambda!(hello)
//! }
//!
//! fn hello(
//!     request: Request,
//!     _ctx: Context
//! ) -> Result<impl IntoResponse, HandlerError> {
//!     Ok(format!(
//!         "hello {}",
//!         request
//!             .query_string_parameters()
//!             .get("name")
//!             .unwrap_or_else(|| "stranger")
//!     ))
//! }
//! ```
//!
//! You can also provide a closure directly to the `lambda!` macro
//!
//! ```rust,no_run
//! use lambda_http::{lambda, Request, RequestExt};
//!
//! fn main() {
//!   lambda!(
//!     |request: Request, context| Ok(
//!       format!(
//!         "hello {}",
//!         request.query_string_parameters()
//!           .get("name")
//!           .unwrap_or_else(|| "stranger")
//!       )
//!     )
//!   )
//! }
//! ```

// only externed because maplit doesn't seem to play well with 2018 edition imports
#[cfg(test)]
#[macro_use]
extern crate maplit;

pub use http::{self, Response};

mod body;
mod ext;
pub mod request;
mod response;
mod strmap;

pub use crate::{
    body::Body, ext::RequestExt, request::LambdaRequest, response::IntoResponse, response::LambdaResponse,
    strmap::StrMap,
};

/// Type alias for `http::Request`s with a fixed `lambda_http::Body` body
// pub struct Request(http::Request<Body>);
pub type Request = http::Request<Body>;
