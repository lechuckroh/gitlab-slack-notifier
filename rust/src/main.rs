extern crate gitlab_slack_notifier;

use lambda_runtime::{Error, LambdaEvent, service_fn};
use serde::{Deserialize, Serialize};

use handlers::HandleEventStatus;
use webhook_events::WebhookEvent;
use crate::handlers::handle_webhook_event;

mod handlers;
mod webhook_events;

#[derive(Deserialize)]
struct Request {
    body: WebhookEvent,
}

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

#[cfg(test)]
mod tests {
    use lambda_runtime::Context;

    use crate::webhook_events::webhook_event_from_file;

    use super::*;

    #[tokio::test]
    async fn test_function_handler() {
        let webhook_event = webhook_event_from_file("../events/issue-open.json").unwrap();
        let lambda_event = LambdaEvent {
            payload: Request { body: webhook_event },
            context: Context::default(),
        };
        let result = function_handler(lambda_event).await.unwrap();
        assert_eq!(result, HandleEventStatus {
            status: "ignored".to_string(),
            message: None,
            error: None,
        });
    }
}
