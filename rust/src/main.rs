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
    use crate::handlers::SlackMessage;

    use crate::webhook_events::webhook_event_from_file;

    use super::*;

    #[tokio::test]
    async fn test_merge_request_open() {
        let webhook_event = webhook_event_from_file("../events/mr-open.json").unwrap();
        let lambda_event = LambdaEvent {
            payload: Request { body: webhook_event },
            context: Context::default(),
        };
        let result = function_handler(lambda_event).await.unwrap();
        assert_eq!(result, HandleEventStatus {
            status: "sent".to_string(),
            message: Some(SlackMessage {
                r#type: "mrkdwn".to_string(),
                text: ":blush: LechuckRoh opened <https://gitlab.com/lechuckroh/gitlab-slack-notifier/-/merge_requests/1|gitlab-slack-notifier MR !1> *Rust Lambda*[`enhancement`].\n`feature/rust` → `develop`".to_string(),
            }),
            error: None,
        });
    }

    #[tokio::test]
    async fn test_merge_request_approve() {
        let webhook_event = webhook_event_from_file("../events/mr-approved.json").unwrap();
        let lambda_event = LambdaEvent {
            payload: Request { body: webhook_event },
            context: Context::default(),
        };
        let result = function_handler(lambda_event).await.unwrap();
        assert_eq!(result, HandleEventStatus {
            status: "sent".to_string(),
            message: Some(SlackMessage {
                r#type: "mrkdwn".to_string(),
                text: ":white_check_mark: LechuckRoh approved <https://gitlab.com/lechuckroh/gitlab-slack-notifier/-/merge_requests/1|gitlab-slack-notifier MR !1> *Rust Lambda*.".to_string(),
            }),
            error: None,
        });
    }

    #[tokio::test]
    async fn test_merge_request_merge() {
        let webhook_event = webhook_event_from_file("../events/mr-merge.json").unwrap();
        let lambda_event = LambdaEvent {
            payload: Request { body: webhook_event },
            context: Context::default(),
        };
        let result = function_handler(lambda_event).await.unwrap();
        assert_eq!(result, HandleEventStatus {
            status: "sent".to_string(),
            message: Some(SlackMessage {
                r#type: "mrkdwn".to_string(),
                text: ":tada: LechuckRoh merged <https://gitlab.com/lechuckroh/gitlab-slack-notifier/-/merge_requests/1|gitlab-slack-notifier MR !1> *Rust Lambda*.".to_string(),
            }),
            error: None,
        });
    }

    #[tokio::test]
    async fn test_pipeline_failed() {
        let webhook_event = webhook_event_from_file("../events/pipeline-failed.json").unwrap();
        let lambda_event = LambdaEvent {
            payload: Request { body: webhook_event },
            context: Context::default(),
        };
        let result = function_handler(lambda_event).await.unwrap();
        assert_eq!(result, HandleEventStatus {
            status: "sent".to_string(),
            message: Some(SlackMessage {
                r#type: "mrkdwn".to_string(),
                text: ":fire: Admin Build pipeline failed on <http://192.168.64.1:3005/gitlab-org/gitlab-test|Gitlab Test project> `master`.\n- `User<user@gitlab.com>` *test*".to_string(),
            }),
            error: None,
        });
    }
}
