use super::service_result::ServiceResult;

pub trait Service<M> {
    fn handle(&mut self, request: M) -> ServiceResult<M>;
}
