use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[allow(clippy::enum_variant_names)]
pub enum ResponsePayload {
    EchoOk { echo: String },
    InitOk,
    GenerateOk { id: String },
}

impl ResponsePayload {
    #[must_use]
    pub fn echo_ok(echo: impl Into<String>) -> Self {
        Self::EchoOk { echo: echo.into() }
    }

    #[must_use]
    pub fn init_ok() -> Self {
        Self::InitOk
    }
}
