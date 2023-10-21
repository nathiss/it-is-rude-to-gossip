use super::{service::Service, service_result::ServiceResult};

pub struct ServicePipe<M> {
    chain: Vec<Box<dyn Service<M>>>,
}

impl<M> ServicePipe<M> {
    pub fn add_service<S>(&mut self, service: S)
    where
        S: Service<M> + 'static,
    {
        let service = Box::new(service);
        self.chain.push(service);
    }

    pub fn process(&mut self, message: M) -> Vec<M>
    where
        M: Clone,
    {
        let mut messages = vec![];
        for service in self.chain.iter_mut() {
            match service.handle(message.clone()) {
                ServiceResult::Response(response) => messages.push(response),
                ServiceResult::Continue => {}
                ServiceResult::Break => break,
            }
        }

        messages
    }
}

impl<M> Default for ServicePipe<M> {
    fn default() -> Self {
        Self {
            chain: Default::default(),
        }
    }
}
