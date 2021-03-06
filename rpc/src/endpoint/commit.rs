//! `/commit` endpoint JSONRPC wrapper

use serde::{Deserialize, Serialize};

use tendermint::{block, block::signed_header::SignedHeader};

/// Get commit information about a specific block
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    height: Option<block::Height>,
}

impl Request {
    /// Create a new request for commit info about a particular block
    pub fn new(height: block::Height) -> Self {
        Self {
            height: Some(height),
        }
    }
}

impl crate::Request for Request {
    type Response = Response;

    fn method(&self) -> crate::Method {
        crate::Method::Commit
    }
}

/// Commit responses
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    /// Signed header
    pub signed_header: SignedHeader,

    /// Is the signed header canonical?
    pub canonical: bool,
}

impl crate::Response for Response {}
