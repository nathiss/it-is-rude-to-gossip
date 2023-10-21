use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use super::{request_payload::RequestPayload, response_payload::ResponsePayload};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Body {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "msg_id")]
    id: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    in_reply_to: Option<usize>,

    #[serde(flatten)]
    payload: Payload,
}

impl Body {
    #[must_use]
    pub fn request(
        id: impl Into<Option<usize>>,
        in_replay_to: impl Into<Option<usize>>,
        request: RequestPayload,
    ) -> Self {
        Self {
            id: id.into(),
            in_reply_to: in_replay_to.into(),
            payload: Payload::Request(request),
        }
    }

    #[must_use]
    pub fn response(
        id: Option<usize>,
        in_replay_to: Option<usize>,
        response: ResponsePayload,
    ) -> Self {
        Self {
            id,
            in_reply_to: in_replay_to,
            payload: Payload::Response(response),
        }
    }

    #[must_use]
    pub fn id(&self) -> &Option<usize> {
        &self.id
    }

    #[must_use]
    pub fn in_replay_to(&self) -> &Option<usize> {
        &self.in_reply_to
    }

    #[must_use]
    pub fn request_payload(&self) -> Option<&RequestPayload> {
        match self.payload {
            Payload::Request(ref payload) => Some(payload),
            Payload::Response(_) => None,
        }
    }

    #[must_use]
    pub fn response_payload(&self) -> Option<&ResponsePayload> {
        match self.payload {
            Payload::Request(_) => None,
            Payload::Response(ref payload) => Some(payload),
        }
    }

    pub(super) fn to_response(&self, message_id: usize) -> Result<Self> {
        let Payload::Request(request) = &self.payload else {
            bail!("Cannot generate response out of `ResponsePayload`.");
        };

        let response_payload = match request {
            RequestPayload::Init { .. } => ResponsePayload::init_ok(),
            RequestPayload::Echo { echo } => ResponsePayload::echo_ok(echo),
            RequestPayload::Generate => ResponsePayload::GenerateOk {
                id: Default::default(),
            },
        };

        Ok(Self {
            id: Some(message_id),
            in_reply_to: self.id,
            payload: Payload::Response(response_payload),
        })
    }

    pub(super) fn to_response_with(
        &self,
        message_id: usize,
        action: impl FnOnce(ResponsePayload) -> ResponsePayload,
    ) -> Result<Self> {
        let Payload::Request(request) = &self.payload else {
            bail!("Cannot generate response out of `ResponsePayload`.");
        };

        let response_payload = match request {
            RequestPayload::Init { .. } => ResponsePayload::init_ok(),
            RequestPayload::Echo { echo } => ResponsePayload::echo_ok(echo),
            RequestPayload::Generate => ResponsePayload::GenerateOk {
                id: Default::default(),
            },
        };
        let response_payload = action(response_payload);

        Ok(Self {
            id: Some(message_id),
            in_reply_to: self.id,
            payload: Payload::Response(response_payload),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum Payload {
    Request(RequestPayload),
    Response(ResponsePayload),
}
