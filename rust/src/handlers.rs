use serde::{Serialize};
use crate::webhook_events::{IssueEvent, MergeRequestEvent, NoteEvent, PipelineEvent, PushEvent, TagPushEvent, WebhookEvent, WikiPageEvent};

#[derive(Serialize, Debug, PartialEq)]
pub struct SlackMessage {
    pub r#type: String,
    pub text: String,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct HandleEventStatus {
    pub status: String,
    pub message: Option<SlackMessage>,
    pub error: Option<String>,
}

impl HandleEventStatus {
    pub fn ignored() -> HandleEventStatus {
        HandleEventStatus {
            status: "ignored".to_string(),
            message: None,
            error: None,
        }
    }
}

fn handle_issue_event(_: IssueEvent) -> HandleEventStatus {
    HandleEventStatus::ignored()
}

fn handle_merge_request_event(_: MergeRequestEvent) -> HandleEventStatus {
    HandleEventStatus::ignored()
}

fn handle_note_event(_: NoteEvent) -> HandleEventStatus {
    HandleEventStatus::ignored()
}

fn handle_pipeline_event(event: PipelineEvent) -> HandleEventStatus {

    HandleEventStatus::ignored()
}

fn handle_push_event(_: PushEvent) -> HandleEventStatus {
    HandleEventStatus::ignored()
}

fn handle_tag_push_event(_: TagPushEvent) -> HandleEventStatus {
    HandleEventStatus::ignored()
}

fn handle_wiki_page_event(_: WikiPageEvent) -> HandleEventStatus {
    HandleEventStatus::ignored()
}

pub async fn handle_webhook_event(event: WebhookEvent) -> Result<HandleEventStatus, &'static str> {
    let resp = match event {
        WebhookEvent::Issue(e) => handle_issue_event(e),
        WebhookEvent::MR(e) => handle_merge_request_event(e),
        WebhookEvent::Note(e) => handle_note_event(e),
        WebhookEvent::Pipeline(e) => handle_pipeline_event(e),
        WebhookEvent::Push(e) => handle_push_event(e),
        WebhookEvent::TagPush(e) => handle_tag_push_event(e),
        WebhookEvent::Wiki(e) => handle_wiki_page_event(e),
    };

    Ok(resp)
}
