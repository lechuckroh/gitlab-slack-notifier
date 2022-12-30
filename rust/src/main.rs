extern crate gitlab_slack_notifier;

use lambda_runtime::{Error, LambdaEvent, service_fn};

use handlers::HandleEventStatus;

use crate::handlers::{handle_webhook_event, Request};

mod handlers;
mod webhook_events;


async fn function_handler(event: LambdaEvent<Request>) -> Result<HandleEventStatus, Error> {
    let webhook_event = event.payload.body;
    let status = handle_webhook_event(webhook_event).await.unwrap();
    Ok(status)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let func = service_fn(function_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

