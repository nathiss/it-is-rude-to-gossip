use crate::{
    context::Context,
    protocol::{Message, RequestPayload},
};

use super::{service::Service, service_result::ServiceResult};

#[derive(Debug)]
pub struct InitService {
    context: Context,
}

impl InitService {
    pub fn new(context: Context) -> Self {
        Self { context }
    }
}

impl Service<Message> for InitService {
    fn handle(&mut self, request: Message) -> ServiceResult<Message> {
        if let Some(RequestPayload::Init { node_id, node_ids }) = request.body().request_payload() {
            self.context.set_node_id(node_id);
            self.context.set_node_ids(node_ids.clone());

            let response = request
                .to_response(self.context.next_id())
                .map(ServiceResult::Response)
                .unwrap_or(ServiceResult::Break);
            return response;
        }

        ServiceResult::Continue
    }
}
