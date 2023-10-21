use crate::{
    context::Context,
    protocol::{Message, RequestPayload},
};

use super::{service::Service, service_result::ServiceResult};

#[derive(Debug)]
pub struct EchoService {
    context: Context,
}

impl EchoService {
    pub fn new(context: Context) -> Self {
        Self { context }
    }
}

impl Service<Message> for EchoService {
    fn handle(&mut self, request: Message) -> ServiceResult<Message> {
        if let Some(RequestPayload::Echo { .. }) = request.body().request_payload() {
            let response = request
                .to_response(self.context.next_id())
                .map(ServiceResult::Response)
                .unwrap_or(ServiceResult::Break);
            return response;
        }

        ServiceResult::Continue
    }
}
