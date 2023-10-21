use crate::{
    context::Context,
    protocol::{Message, RequestPayload, ResponsePayload},
};

use super::{service::Service, service_result::ServiceResult};

#[derive(Debug)]
pub struct UniqueIdService {
    context: Context,
}

impl UniqueIdService {
    pub fn new(context: Context) -> Self {
        Self { context }
    }
}

impl Service<Message> for UniqueIdService {
    fn handle(&mut self, request: Message) -> ServiceResult<Message> {
        if let Some(RequestPayload::Generate) = request.body().request_payload() {
            let self_id = self.context.get_node_id();
            let id = self.context.next_id();
            let unique_id = format!("{self_id}-{id}");

            let response = request
                .to_response_with(self.context.next_id(), |p| match p {
                    ResponsePayload::GenerateOk { .. } => {
                        ResponsePayload::GenerateOk { id: unique_id }
                    }
                    _ => p,
                })
                .map(ServiceResult::Response)
                .unwrap_or(ServiceResult::Break);
            return response;
        }

        ServiceResult::Continue
    }
}
