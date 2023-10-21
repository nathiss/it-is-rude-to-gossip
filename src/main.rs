mod context;
mod protocol;
mod services;

use std::{fs::File, io::Write, sync::mpsc::channel};

use anyhow::Context;
use services::{EchoService, InitService, ServicePipe, UniqueIdService};

use crate::protocol::Message;

fn setup_logger() {
    let log_file = File::create("./log.txt").unwrap();
    let log_file = Box::new(log_file);

    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .target(env_logger::Target::Stderr)
        .target(env_logger::Target::Pipe(log_file))
        .init();
}

fn create_service_chain(context: context::Context) -> ServicePipe<Message> {
    let mut services = ServicePipe::default();
    services.add_service(InitService::new(context.clone()));
    services.add_service(EchoService::new(context.clone()));
    services.add_service(UniqueIdService::new(context.clone()));

    services
}

fn main() -> anyhow::Result<()> {
    setup_logger();

    let (tx, rx) = channel();

    let handle = std::thread::spawn(move || -> anyhow::Result<()> {
        let mut stdout = std::io::stdout().lock();
        while let Ok(msg) = rx.recv() {
            serde_json::to_writer(&mut stdout, &msg)?;
            stdout.write_all(b"\n")?;
            stdout.flush()?;
        }

        Ok(())
    });

    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let context = context::Context::default();
    let mut services = create_service_chain(context);

    for message in inputs {
        let message = message.context("Could not deserialize server's message.")?;
        log::debug!("Deserialized a message: {message:?}");
        for response in services.process(message) {
            tx.send(response)?;
        }
    }

    handle.join().unwrap()?;

    Ok(())
}
