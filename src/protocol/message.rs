use std::convert::identity;

use super::{body::Body, ResponsePayload};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    src: String,
    dest: String,
    body: Body,
}

impl Message {
    #[must_use]
    pub fn new(src: impl Into<String>, dest: impl Into<String>, body: impl Into<Body>) -> Self {
        Self {
            src: src.into(),
            dest: dest.into(),
            body: body.into(),
        }
    }

    pub fn to_response(&self, message_id: usize) -> Result<Self> {
        self.to_response_with(message_id, identity)
    }

    pub fn to_response_with(
        &self,
        message_id: usize,
        action: impl FnOnce(ResponsePayload) -> ResponsePayload,
    ) -> Result<Self> {
        Ok(Self {
            src: self.dest.clone(),
            dest: self.src.clone(),
            body: self.body.to_response_with(message_id, action)?,
        })
    }

    #[must_use]
    pub fn src(&self) -> &str {
        &self.src
    }

    #[must_use]
    pub fn dest(&self) -> &str {
        &self.dest
    }

    #[must_use]
    pub fn body(&self) -> &Body {
        &self.body
    }
}
