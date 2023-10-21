use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum RequestPayload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    Echo {
        echo: String,
    },
    Generate,
}

impl RequestPayload {
    #[must_use]
    pub fn init(node_id: impl Into<String>, node_ids: impl Into<Vec<String>>) -> Self {
        Self::Init {
            node_id: node_id.into(),
            node_ids: node_ids.into(),
        }
    }

    #[must_use]
    pub fn echo(echo: impl Into<String>) -> Self {
        Self::Echo { echo: echo.into() }
    }
}
