use serde::{Serialize};
use crate::webhook_events::{IssueEvent, MergeRequestEvent, NoteEvent, PipelineEvent, PushEvent, TagPushEvent, WebhookEvent, WikiPageEvent};

#[derive(Serialize, Debug, PartialEq)]
pub struct HandleEventStatus {
    pub status: String,
    pub message: Option<String>,
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

async fn handle_issue_event(_: IssueEvent) -> Result<HandleEventStatus, &'static str> {
    Ok(HandleEventStatus::ignored())
}

async fn handle_merge_request_event(_: MergeRequestEvent) -> Result<HandleEventStatus, &'static str> {
    Ok(HandleEventStatus::ignored())
}

async fn handle_note_event(_: NoteEvent) -> Result<HandleEventStatus, &'static str> {
    Ok(HandleEventStatus::ignored())
}

async fn handle_pipeline_event(_: PipelineEvent) -> Result<HandleEventStatus, &'static str> {
    Ok(HandleEventStatus::ignored())
}

async fn handle_push_event(_: PushEvent) -> Result<HandleEventStatus, &'static str> {
    Ok(HandleEventStatus::ignored())
}

async fn handle_tag_push_event(_: TagPushEvent) -> Result<HandleEventStatus, &'static str> {
    Ok(HandleEventStatus::ignored())
}

async fn handle_wiki_page_event(_: WikiPageEvent) -> Result<HandleEventStatus, &'static str> {
    Ok(HandleEventStatus::ignored())
}

pub async fn handle_webhook_event(event: WebhookEvent) -> Result<HandleEventStatus, &'static str> {
    let resp = match event {
        WebhookEvent::Issue(e) => handle_issue_event(e).await.unwrap(),
        WebhookEvent::MR(e) => handle_merge_request_event(e).await.unwrap(),
        WebhookEvent::Note(e) => handle_note_event(e).await.unwrap(),
        WebhookEvent::Pipeline(e) => handle_pipeline_event(e).await.unwrap(),
        WebhookEvent::Push(e) => handle_push_event(e).await.unwrap(),
        WebhookEvent::TagPush(e) => handle_tag_push_event(e).await.unwrap(),
        WebhookEvent::Wiki(e) => handle_wiki_page_event(e).await.unwrap(),
    };
    Ok(resp)
}
